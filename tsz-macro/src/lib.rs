use itertools::izip;
use itertools::{multiunzip, Itertools};
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Meta};

#[proc_macro_derive(DeltaEncodable)]
pub fn derive_delta_encodable(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::DeriveInput);

    let ident = input.ident.clone();
    let delta_ident = format_ident!("{}Delta", input.ident);
    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        _ => panic!("Expected fields in derive(Builder) struct"),
    };
    let named_fields = match fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => named,
        _ => panic!("Expected named fields in derive(Builder) struct"),
    };
    let fields = named_fields
        .into_iter()
        .map(|f| (f.ident.unwrap(), f.ty))
        .collect::<Vec<_>>();
    let delta_field_names = fields.iter().map(|(name, _)| name).collect::<Vec<_>>();
    let delta_field_types = fields
        .iter()
        .map(|(_, ty)| {
            // Find the next highest type that can represent the delta.
            // If non primitive, then panic.
            // i8 -> i16, i16 -> i32, i32 -> i64, i64 -> i128, i128 -> i128
            // u8 -> i16, u16 -> i32, u32 -> i64, u64 -> i128, u128 -> i128

            match ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { i16 },
                        "i16" => quote! { i32 },
                        "i32" => quote! { i64 },
                        "i64" => quote! { i128 },
                        "i128" => quote! { i128 },
                        "u8" => quote! { i16 },
                        "u16" => quote! { i32 },
                        "u32" => quote! { i64 },
                        "u64" => quote! { i128 },
                        "u128" => quote! { i128 },
                        _ => panic!("Unsupported type"),
                    }
                }
                _ => panic!("Unsupported type"),
            }

            // ty
        })
        .collect::<Vec<_>>();

    let field_types = fields.iter().map(|(_, ty)| ty).collect::<Vec<_>>();

    quote! {
        #[derive(Clone, Copy, Debug)]
        pub struct #delta_ident {
            #( #delta_field_names: #delta_field_types ),*
        }

        impl ::core::ops::Sub for #ident {
            type Output = #delta_ident;

            fn sub(self, rhs: Self) -> Self::Output {
                #delta_ident {
                    #( #delta_field_names: self.#delta_field_names as #delta_field_types - rhs.#delta_field_names as #delta_field_types),*
                }
            }
        }

        impl ::core::ops::Add<#delta_ident> for #ident {
            type Output = #ident;

            fn add(self, rhs: #delta_ident) -> Self::Output {
                #ident {
                    #( #delta_field_names: (self.#delta_field_names as #delta_field_types + rhs.#delta_field_names) as #field_types),*
                }
            }
        }

        impl ::core::ops::Sub for #delta_ident {
            type Output = #delta_ident;

            fn sub(self, rhs: Self) -> Self::Output {
                #delta_ident {
                    #( #delta_field_names: self.#delta_field_names - rhs.#delta_field_names),*
                }
            }
        }
    }
    .into()
}

#[proc_macro_derive(Compressible)]
pub fn derive_compressible(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::DeriveInput);

    let ident = input.ident.clone();
    let delta_ident = format_ident!("{}Delta", input.ident);
    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        _ => panic!("Expected fields in derive(Builder) struct"),
    };
    let named_fields = match fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => named,
        _ => panic!("Expected named fields in derive(Builder) struct"),
    };
    let fields = named_fields
        .into_iter()
        .map(|f| (f.ident.unwrap(), f.ty))
        .collect::<Vec<_>>();
    let delta_field_names = fields.iter().map(|(name, _)| name).collect::<Vec<_>>();
    let delta_field_types = fields
        .iter()
        .map(|(_, ty)| {
            // Find the next highest type that can represent the delta.
            // If non primitive, then panic.
            // i8 -> i16, i16 -> i32, i32 -> i64, i64 -> i128, i128 -> i128
            // u8 -> i16, u16 -> i32, u32 -> i64, u64 -> i128, u128 -> i128

            match ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { i16 },
                        "i16" => quote! { i32 },
                        "i32" => quote! { i64 },
                        "i64" => quote! { i128 },
                        "i128" => quote! { i128 },
                        "u8" => quote! { i16 },
                        "u16" => quote! { i32 },
                        "u32" => quote! { i64 },
                        "u64" => quote! { i128 },
                        "u128" => quote! { i128 },
                        _ => panic!("Unsupported type"),
                    }
                }
                _ => panic!("Unsupported type"),
            }

            // ty
        })
        .collect::<Vec<_>>();
    let delta_field_encoded_types = fields
        .iter()
        .map(|(_, ty)| {
            // Find the next highest type that can represent the delta.
            // If non primitive, then panic.
            // i8 -> i16, i16 -> i32, i32 -> i64, i64 -> i64, i128 -> i64

            match ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { i16 },
                        "i16" => quote! { i32 },
                        "i32" => quote! { i64 },
                        "i64" => quote! { i64 },
                        "i128" => quote! { i64 },
                        _ => panic!("Unsupported type"),
                    }
                }
                _ => panic!("Unsupported type"),
            }

            // ty
        })
        .collect::<Vec<_>>();

    let vlq_types = fields
        .iter()
        .map(|(_, ty)| {
            // Signed values will use tsz_compress::compress::Svlq, unsigned values will use tsz_compress::compress::Uvlq.

            match ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { tsz_compress::svlq::Svlq },
                        "i16" => quote! { tsz_compress::svlq::Svlq },
                        "i32" => quote! { tsz_compress::svlq::Svlq },
                        "i64" => quote! { tsz_compress::svlq::Svlq },
                        "i128" => quote! { tsz_compress::svlq::Svlq },
                        "u8" => quote! { tsz_compress::uvlq ::Uvlq },
                        "u16" => quote! { tsz_compress::uvlq::Uvlq },
                        "u32" => quote! { tsz_compress::uvlq::Uvlq },
                        "u64" => quote! { tsz_compress::uvlq::Uvlq },
                        "u128" => quote! { tsz_compress::uvlq::Uvlq },
                        _ => panic!("Unsupported type"),
                    }
                }
                _ => panic!("Unsupported type"),
            }

            // ty
        })
        .collect::<Vec<_>>();

    // All i128 columns need to check if the values are out of supported bounds.
    let encode_delta_fn_calls = delta_field_names.iter().zip(delta_field_types.iter().zip(delta_field_encoded_types.iter()))
    .map(| (field_name, (field_ty, encoded_field_ty))| {
        // if the field_ty is i128, then encoded_field_ty will be i64
        // check if the field is in bounds of i64::MIN and i64::MAX for those fields

        let encode_fn_name = format_ident!("encode_delta_{}", encoded_field_ty.to_string().to_lowercase());
        let field_ty_name =syn::parse2::<syn::Type>(field_ty.clone()).unwrap();
        match field_ty_name {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let segment = path.segments.first().unwrap();
                let ident = segment.ident.clone();
                match ident.to_string().as_str() {
                    "i128" => {
                        quote! {
                            if self.#field_name < i64::MIN as i128 || self.#field_name > i64::MAX as i128 {
                                unimplemented!();
                            }
                            tsz_compress::delta::#encode_fn_name(self.#field_name as i64, out);
                        }
                    },
                    _ => {
                        quote! {
                            tsz_compress::delta::#encode_fn_name(self.#field_name, out);
                        }
                    }
                }
            }
            _ => panic!("Unsupported type"),
        }
    })
    .collect::<Vec<_>>();

    quote! {

        impl IntoCompressBits for #ident {
            fn into_bits(self, out: &mut tsz_compress::prelude::BitBuffer) {
                #( out.extend(#vlq_types::from(self.#delta_field_names).bits); )*
            }
        }

        impl IntoCompressBits for #delta_ident {
            fn into_bits(self, out: &mut tsz_compress::prelude::BitBuffer) {
                #(
                    #encode_delta_fn_calls
                )*
            }
        }

        impl Compress for #ident {
            type Full = #ident;
            type Delta = #delta_ident;

            fn into_full(self) -> Self::Full {
                self
            }

            fn into_delta(self, prev: &Self::Full) -> Self::Delta {
                self - *prev
            }

            fn into_deltadelta(self, prev_prev_row: &Self, prev_row: &Self) -> Self::Delta {
                (self - *prev_row) - (*prev_row - *prev_prev_row)
            }
        }
    }
    .into()
}

#[proc_macro_derive(Decompressible)]
pub fn derive_decompressible(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::DeriveInput);

    let ident = input.ident.clone();
    let delta_ident = format_ident!("{}Delta", input.ident);
    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        _ => panic!("Expected fields in derive(Builder) struct"),
    };
    let named_fields = match fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => named,
        _ => panic!("Expected named fields in derive(Builder) struct"),
    };
    let fields = named_fields
        .into_iter()
        .map(|f| (f.ident.unwrap(), f.ty))
        .collect::<Vec<_>>();
    let delta_field_names = fields.iter().map(|(name, _)| name).collect::<Vec<_>>();
    let delta_field_types = fields
        .iter()
        .map(|(_, ty)| {
            // Find the next highest type that can represent the delta.
            // If non primitive, then panic.
            // i8 -> i16, i16 -> i32, i32 -> i64, i64 -> i128, i128 -> i128
            // u8 -> i16, u16 -> i32, u32 -> i64, u64 -> i128, u128 -> i128

            match ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { i16 },
                        "i16" => quote! { i32 },
                        "i32" => quote! { i64 },
                        "i64" => quote! { i128 },
                        "i128" => quote! { i128 },
                        "u8" => quote! { i16 },
                        "u16" => quote! { i32 },
                        "u32" => quote! { i64 },
                        "u64" => quote! { i128 },
                        "u128" => quote! { i128 },
                        _ => panic!("Unsupported type"),
                    }
                }
                _ => panic!("Unsupported type"),
            }

            // ty
        })
        .collect::<Vec<_>>();
    let delta_field_encoded_types = fields
        .iter()
        .map(|(_, ty)| {
            // Find the next highest type that can represent the delta.
            // If non primitive, then panic.
            // i8 -> i16, i16 -> i32, i32 -> i64, i64 -> i64, i128 -> i64

            match ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { i16 },
                        "i16" => quote! { i32 },
                        "i32" => quote! { i64 },
                        "i64" => quote! { i64 },
                        "i128" => quote! { i64 },
                        _ => panic!("Unsupported type"),
                    }
                }
                _ => panic!("Unsupported type"),
            }

            // ty
        })
        .collect::<Vec<_>>();

    let field_types = fields.iter().map(|(_, ty)| ty).collect::<Vec<_>>();

    let vlq_ref_types = fields
        .iter()
        .map(|(_, ty)| {
            // Signed values will use tsz_compress::compress::Svlq, unsigned values will use tsz_compress::compress::Uvlq.

            match ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { tsz_compress::svlq::SvlqRef },
                        "i16" => quote! { tsz_compress::svlq::SvlqRef },
                        "i32" => quote! { tsz_compress::svlq::SvlqRef },
                        "i64" => quote! { tsz_compress::svlq::SvlqRef },
                        "i128" => quote! { tsz_compress::svlq::SvlqRef },
                        "u8" => quote! { tsz_compress::uvlq::UvlqRef },
                        "u16" => quote! { tsz_compress::uvlq::UvlqRef },
                        "u32" => quote! { tsz_compress::uvlq::UvlqRef },
                        "u64" => quote! { tsz_compress::uvlq::UvlqRef },
                        "u128" => quote! { tsz_compress::uvlq::UvlqRef },
                        _ => panic!("Unsupported type"),
                    }
                }
                _ => panic!("Unsupported type"),
            }

            // ty
        })
        .collect::<Vec<_>>();

    // functions to call for the typ like, decode_delta_i8, decode_delta_i16, etc.
    let decode_delta_fns = delta_field_encoded_types
        .iter()
        .map(|type_token_stream| {
            // parse the type token stream to the type
            let ty = syn::parse2::<syn::Type>(type_token_stream.clone()).unwrap();
            match ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { decode_delta_i8 },
                        "i16" => quote! { decode_delta_i16 },
                        "i32" => quote! { decode_delta_i32 },
                        "i64" => quote! { decode_delta_i64 },
                        _ => panic!("Unsupported type to delta encode/decode"),
                    }
                }
                _ => panic!("Unsupported type"),
            }
        })
        .collect::<Vec<_>>();

    // All but the last call should include a check for early EOF.
    // #(
    //     let (#delta_field_names, input) = #decode_delta_fns(input)?;
    //     let Some(input) = input else {
    //         return Err("Early EOF");
    //     };
    // )*

    let decode_delta_fn_calls = delta_field_names
        .iter()
        .zip(decode_delta_fns.iter())
        .enumerate()
        .map(|(idx, (field_name, fn_name))| {
            if idx != decode_delta_fns.len() - 1 {
                quote! {
                    let (#field_name, input) = tsz_compress::delta::#fn_name(input)?;
                    let Some(input) = input else {
                        return Err("Early EOF");
                    };
                }
            } else {
                quote! {
                    let (#field_name, input) = tsz_compress::delta::#fn_name(input)?;
                    let input = input.unwrap_or_default();
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        impl FromCompressBits for #ident {
            fn from_bits(input: &tsz_compress::prelude::BitBufferSlice) -> Result<(Self, &tsz_compress::prelude::BitBufferSlice), &'static str> {
                #(
                    let (#delta_field_names, read) = <(#field_types, usize)>::try_from(#vlq_ref_types(input))?;
                    let input = &input[read..];
                )*

                Ok((Self {
                    #( #delta_field_names, )*
                }, input))
            }
        }

        impl FromCompressBits for #delta_ident {
            fn from_bits(input: &tsz_compress::prelude::BitBufferSlice) -> Result<(Self, &tsz_compress::prelude::BitBufferSlice), &'static str> {
                #(
                    #decode_delta_fn_calls
                )*

                Ok((Self {
                    #( #delta_field_names: #delta_field_names as #delta_field_types, )*
                }, input))
            }
        }

        impl Decompress for #ident {
            type Full = #ident;
            type Delta = #delta_ident;

            fn from_full<'a>(bits: &'a tsz_compress::prelude::BitBufferSlice) -> Result<(Self, &'a tsz_compress::prelude::BitBufferSlice), &'static str> {
                #ident::from_bits(bits).map_err(|_| "failed to unmarshal full row")
            }

            fn from_delta<'a>(bits: &'a tsz_compress::prelude::BitBufferSlice, prev_row: &Self) -> Result<(Self, &'a tsz_compress::prelude::BitBufferSlice), &'static str> {
                let delta = #delta_ident::from_bits(bits).map_err(|_| "failed to unmarshal delta row")?;
                Ok((*prev_row + delta.0, delta.1))
            }

            fn from_deltadelta<'a>(bits: &'a tsz_compress::prelude::BitBufferSlice, prev_row: &Self, prev_prev_row: &Self) -> Result<(Self, &'a tsz_compress::prelude::BitBufferSlice), &'static str> {
                let deltadelta = #delta_ident::from_bits(bits).map_err(|_| "failed to unmarshal deltadelta row")?;
                Ok((*prev_row + (*prev_row - *prev_prev_row) + deltadelta.0, deltadelta.1))
            }
        }
    }
    .into()
}

fn get_fields_of_struct(input: syn::DeriveInput) -> Vec<(syn::Ident, syn::Type, Option<String>)> {
    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        _ => panic!("Expected fields in derive(Builder) struct"),
    };
    let named_fields = match fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => named,
        _ => panic!("Expected named fields in derive(Builder) struct"),
    };

    // Get the tsz attributes for each field attribute
    let delta_attributes = named_fields
        .iter()
        .map(|field| {
            let filtered_attrs: Vec<_> = field
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident("tsz"))
                .collect();
            Option::from(filtered_attrs).filter(|v| !v.is_empty())
        })
        .collect::<Vec<_>>();

    // Get delta column types from each tsz field attribute
    let mut delta_user_col_tys: Vec<Option<String>> = Vec::new();
    for delta_attribute in delta_attributes {
        if let Some(delta_attr) = delta_attribute {
            for attr in delta_attr {
                // There should only be one tsz attribute per field: delta
                if let Meta::List(meta_list) = attr.meta.clone() {
                    let tokens = meta_list.tokens.into_iter().peekable();
                    let mut identifier = String::new();
                    let mut punct = String::new();
                    let mut literal = String::new();

                    for token in tokens {
                        if let TokenTree::Ident(ident) = &token {
                            identifier = ident.to_string();
                        } else if let TokenTree::Punct(p) = &token {
                            punct = p.to_string();
                        } else if let TokenTree::Literal(lit) = &token {
                            literal = lit.to_string();
                        }
                    }

                    match (identifier.as_str(), punct.as_str()) {
                        ("delta", "=") => delta_user_col_tys.push(Some(literal)),
                        ("delta", _) => panic!("Unexpected field operator"),
                        _ => panic!("Unexpected delta bit-width attribute"),
                    }
                }
            }
        } else {
            delta_user_col_tys.push(None);
        }
    }

    named_fields
        .into_iter()
        .enumerate()
        .map(|(i, f)| {
            let attr = &delta_user_col_tys[i];
            (f.ident.unwrap(), f.ty, attr.clone())
        })
        .collect::<Vec<_>>() // (ident, ty, delta_bit_width)
}

///
/// CompressV2 is a procedural macro that will inspect the fields of
/// a struct and generate a StructCompressor with statically sized columnar
/// compression for the fields.
///
#[proc_macro_derive(CompressV2, attributes(tsz))]
pub fn derive_compressv2(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as syn::DeriveInput);
    let ident = input.ident.clone();

    // We will define a struct by this name
    let compressor_ident = format_ident!("{}CompressorImpl", input.ident);

    // We will compress each of the fields as columns
    let columns = get_fields_of_struct(input);
    let (col_idents, col_tys, col_attrs): (Vec<_>, Vec<_>, Vec<_>) = multiunzip(columns);
    let col_delta_comp_queue_idents = col_idents
        .iter()
        .map(|ident| format_ident!("{}_delta_compressor_queue", ident))
        .collect_vec();
    let col_delta_delta_comp_queue_idents = col_idents
        .iter()
        .map(|ident| format_ident!("{}_delta_delta_compressor_queue", ident))
        .collect_vec();
    let col_delta_buf_idents = col_idents
        .iter()
        .map(|ident| format_ident!("{}_delta_output_buffer", ident))
        .collect_vec();
    let col_delta_delta_buf_idents = col_idents
        .iter()
        .map(|ident| format_ident!("{}_delta_delta_output_buffer", ident))
        .collect_vec();
    let num_columns = col_idents.len();

    // Get the delta types for each column: If user specified, use that, otherwise use default
    let delta_col_tys = col_attrs
        .iter()
        .zip(&col_tys)
        .map(|(attr, ty)| match attr.as_ref() {
            Some(s) if s == "\"i8\"" => quote! { i8 },
            Some(s) if s == "\"i16\"" => quote! { i16 },
            Some(s) if s == "\"i32\"" => quote! { i32 },
            Some(s) if s == "\"i64\"" => quote! { i64 },
            None => match ty {
                // Default Deltas
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { i16 },
                        "i16" => quote! { i32 },
                        "i32" => quote! { i64 },
                        "i64" => quote! { i64 },
                        _ => panic!("Unsupported type"),
                    }
                }
                _ => panic!("Unsupported type"),
            },
            _ => panic!("Unsupported type"),
        })
        .collect::<Vec<_>>();

    let double_col_tys = col_tys
        .iter()
        .map(|ty| match ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let segment = path.segments.first().unwrap();
                let ident = segment.ident.clone();
                match ident.to_string().as_str() {
                    "i8" => quote! { i16 },
                    "i16" => quote! { i32 },
                    "i32" => quote! { i64 },
                    "i64" => quote! { i128 },
                    "i128" => quote! { i128 }, // Note i128 is not doubled
                    _ => panic!("Unsupported type"),
                }
            }
            _ => panic!("Unsupported type"),
        })
        .collect::<Vec<_>>();

    // todo, make choice based on macro attributes for field, default to delta-delta and delta
    let col_delta_buf = col_tys
        .iter()
        .map(|ty| match ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let segment = path.segments.first().unwrap();
                let ident = segment.ident.clone();
                match ident.to_string().as_str() {
                    "i8" => quote! { Some(::tsz_compress::prelude::halfvec::HalfVec::new(prealloc_rows)) },
                    "i16" => quote! { Some(::tsz_compress::prelude::halfvec::HalfVec::new(prealloc_rows)) },
                    "i32" => quote! { Some(::tsz_compress::prelude::halfvec::HalfVec::new(prealloc_rows)) },
                    "i64" => quote! { Some(::tsz_compress::prelude::halfvec::HalfVec::new(prealloc_rows)) },
                    // "i64" => quote! { None },
                    "i128" => quote! { None },
                    _ => panic!("Unsupported type"),
                }
            }
            _ => panic!("Unsupported type"),
        })
        .collect::<Vec<_>>();
    let col_delta_delta_buf = col_tys
        .iter()
        .map(|ty| match ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let segment = path.segments.first().unwrap();
                let ident = segment.ident.clone();
                match ident.to_string().as_str() {
                    "i8" => quote! { None },
                    "i16" => quote! { None },
                    "i32" => quote! { None },
                    "i64" => quote! { None },
                    // "i64" => quote! { Some(::tsz_compress::prelude::halfvec::HalfVec::new(prealloc_rows)) },
                    "i128" => quote! { Some(::tsz_compress::prelude::halfvec::HalfVec::new(prealloc_rows)) },
                    _ => panic!("Unsupported type"),
                }
            }
            _ => panic!("Unsupported type"),
        })
        .collect::<Vec<_>>();

    let write_first = col_tys
        .iter()
        .map(|ty| match ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let segment = path.segments.first().unwrap();
                let ident = segment.ident.clone();
                match ident.to_string().as_str() {
                    "i8" => quote! { ::tsz_compress::prelude::write_i8_bits },
                    "i16" => quote! { ::tsz_compress::prelude::write_i16_bits },
                    "i32" => quote! { ::tsz_compress::prelude::write_i32_bits },
                    "i64" => quote! { ::tsz_compress::prelude::write_i64_bits },
                    "i128" => quote! { ::tsz_compress::prelude::write_i128_bits },
                    _ => panic!("Unsupported type"),
                }
            }
            _ => panic!("Unsupported type"),
        })
        .collect::<Vec<_>>();
    let write_second = col_tys
        .iter()
        .map(|ty| match ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let segment = path.segments.first().unwrap();
                let ident = segment.ident.clone();
                match ident.to_string().as_str() {
                    "i8" => quote! { ::tsz_compress::prelude::write_i16_bits },
                    "i16" => quote! { ::tsz_compress::prelude::write_i32_bits },
                    "i32" => quote! { ::tsz_compress::prelude::write_i64_bits },
                    "i64" => quote! { ::tsz_compress::prelude::write_i128_bits },
                    "i128" => quote! { ::tsz_compress::prelude::write_i128_bits },
                    _ => panic!("Unsupported type"),
                }
            }
            _ => panic!("Unsupported type"),
        })
        .collect::<Vec<_>>();
    let prev_col_idents = col_idents
        .iter()
        .map(|ident| format_ident!("prev_{}", ident))
        .collect_vec();
    let prev_double_col_idents = col_idents
        .iter()
        .map(|ident| format_ident!("prev_double_{}", ident))
        .collect_vec();
    let prev_delta_idents = col_idents
        .iter()
        .map(|ident| format_ident!("prev_delta_{}", ident))
        .collect_vec();

    // Do delta compression
    let delta_comp_block = izip!(col_tys.iter(), col_delta_buf_idents.iter(), col_delta_comp_queue_idents.iter())
        .map(|(ty, col_delta_buf_idents,  col_delta_comp_queue_idents)|  match ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let segment = path.segments.first().unwrap();
                let ident = segment.ident.clone();
                match ident.to_string().as_str() {
                    "i8" => quote! {
                        debug_assert!(self.#col_delta_buf_idents.is_some());
                        let outbuf = unsafe { self.#col_delta_buf_idents.as_mut().unwrap_unchecked() };
                        self.#col_delta_comp_queue_idents.push(delta);
                        if self.#col_delta_comp_queue_idents.is_full() {
                            self.#col_delta_comp_queue_idents.emit_delta_bits(outbuf);
                        }
                    },
                    "i16" => quote! {
                        debug_assert!(self.#col_delta_buf_idents.is_some());
                        let outbuf = unsafe { self.#col_delta_buf_idents.as_mut().unwrap_unchecked() };
                        self.#col_delta_comp_queue_idents.push(delta);
                        if self.#col_delta_comp_queue_idents.is_full() {
                            self.#col_delta_comp_queue_idents.emit_delta_bits(outbuf);
                        }
                    },
                    "i32" => quote! {
                        debug_assert!(self.#col_delta_buf_idents.is_some());
                        let outbuf = unsafe { self.#col_delta_buf_idents.as_mut().unwrap_unchecked() };
                        self.#col_delta_comp_queue_idents.push(delta);
                        if self.#col_delta_comp_queue_idents.is_full() {
                            self.#col_delta_comp_queue_idents.emit_delta_bits(outbuf);
                        }
                    },
                    "i64" => quote! {
                        debug_assert!(self.#col_delta_buf_idents.is_some());
                        let outbuf = unsafe { self.#col_delta_buf_idents.as_mut().unwrap_unchecked() };
                        self.#col_delta_comp_queue_idents.push(delta);
                        if self.#col_delta_comp_queue_idents.is_full() {
                            self.#col_delta_comp_queue_idents.emit_delta_bits(outbuf);
                        }
                    },
                    "i128" => quote! { },
                    _ => panic!("Unsupported type"),
                }
            }
            _ => panic!("Unsupported type"),
        })
        .collect::<Vec<_>>();

    // Do delta-delta compression
    let delta_delta_comp_block = izip!(col_tys.iter(), col_delta_delta_buf_idents.iter(), prev_delta_idents.iter(), col_delta_delta_comp_queue_idents.iter())
        .map(|(ty, col_delta_delta_buf_idents, prev_delta_idents, col_delta_delta_comp_queue_idents)|  match ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let segment = path.segments.first().unwrap();
                let ident = segment.ident.clone();
                match ident.to_string().as_str() {
                    "i8" => quote! { },
                    "i16" => quote! { },
                    "i32" => quote! { },
                    "i64" => quote! { },
                    "i128" => quote! {
                        debug_assert!(self.#col_delta_delta_buf_idents.is_some());
                        let outbuf = unsafe { self.#col_delta_delta_buf_idents.as_mut().unwrap_unchecked() };
                        let delta_delta = delta - self.#prev_delta_idents;
                        self.#col_delta_delta_comp_queue_idents.push(delta_delta);
                        if self.#col_delta_delta_comp_queue_idents.is_full() {
                            self.#col_delta_delta_comp_queue_idents.emit_delta_delta_bits(outbuf);
                        }
                    },
                    _ => panic!("Unsupported type"),
                }
            }
            _ => panic!("Unsupported type"),
        })
        .collect::<Vec<_>>();

    let finish_into_thin = if cfg!(feature = "thin-vec") {
        quote! {
            ///
            /// Consumes the compressor state, appending compressed bytes
            /// to the provided buffer and reserving space if needed.
            ///
            /// Leaving the intermediate buffers in a reserved, cleared state.
            ///
            fn finish_into_thin(&mut self, output_bytes: &mut ::thin_vec::ThinVec<u8>) {
                // Only use one encoding mechanism
                #(
                    if let (Some(delta_buffer), Some(delta_delta_buffer)) = (&self.#col_delta_buf_idents, &self.#col_delta_delta_buf_idents) {
                        // Prefer delta on ties
                        if delta_delta_buffer.len() >= delta_buffer.len() {
                            self.#col_delta_delta_buf_idents = None;
                        } else {
                            self.#col_delta_buf_idents = None;
                        }
                    }
                )*

                // Guarantee that at least the column start nibble is emitted
                #(
                    if let Some(outbuf) = self.#col_delta_buf_idents.as_mut() {
                        if outbuf.is_empty() {
                            outbuf.push(::tsz_compress::prelude::halfvec::HalfWord::Half(::tsz_compress::prelude::consts::headers::START_OF_COLUMN));
                        }
                    }
                    if let Some(outbuf) = self.#col_delta_delta_buf_idents.as_mut() {
                        if outbuf.is_empty() {
                            outbuf.push(::tsz_compress::prelude::halfvec::HalfWord::Half(::tsz_compress::prelude::consts::headers::START_OF_COLUMN));
                        }
                    }
                )*

                // Flush any pending samples in the queues
                // All of the bits are concatenated with a 1001 tag indicating the start of a new column
                #(
                    self.#col_delta_buf_idents.as_mut().map(|outbuf| {
                        while self.#col_delta_comp_queue_idents.len() > 0 {
                            self.#col_delta_comp_queue_idents.flush_delta_bits(outbuf);
                        }
                        });
                    self.#col_delta_delta_buf_idents.as_mut().map(|outbuf| {
                        while self.#col_delta_delta_comp_queue_idents.len() > 0 {
                            self.#col_delta_delta_comp_queue_idents.emit_delta_delta_bits(outbuf);
                        }
                    });
                )*

                // Write the number of rows as a 32-bit integer
                // The decompressor will read this value and reserve space for the rows
                // SAFETY: The number of rows may be more than 2^32, but the decompressor will
                //         reserve at most 2^32 rows.
                let mut rows = ::tsz_compress::prelude::halfvec::HalfVec::new(8);
                ::tsz_compress::prelude::write_i32_bits(&mut rows, self.rows as u32 as i32);

                // Create an iterator over the words to be written
                let rows = Some(rows);
                let words = [
                    rows.as_ref().into_iter(),
                    #(
                        self.#col_delta_buf_idents.as_ref().into_iter(),
                        self.#col_delta_delta_buf_idents.as_ref().into_iter(),
                    )*
                ].into_iter().flatten();

                // Pack the words into nibbles
                ::tsz_compress::prelude::halfvec::HalfVec::finish_thin(output_bytes, words);

                // Clear the buffers for re-use
                #(
                    self.#col_delta_buf_idents.as_mut().map(|outbuf| {
                        outbuf.clear();
                    });
                    self.#col_delta_delta_buf_idents.as_mut().map(|outbuf| {
                        outbuf.clear();
                    });
                    self.rows = 0;
                )*
            }
        }
    } else {
        quote! {}
    };

    let compressor_struct = quote! {
        pub mod compress {
            use super::*;
            mod private {
                use super::*;
                use ::tsz_compress::prelude::*;
                /// A Compressor type implementing TszCompressV2.
                #[derive(Debug)]
                pub struct #compressor_ident {
                    #( #col_delta_comp_queue_idents: ::tsz_compress::prelude::CompressionQueue<10>,)*
                    #( #col_delta_delta_comp_queue_idents: ::tsz_compress::prelude::CompressionQueue<2>,)*
                    #( #col_delta_buf_idents: Option<::tsz_compress::prelude::halfvec::HalfVec>,)*
                    #( #col_delta_delta_buf_idents: Option<::tsz_compress::prelude::halfvec::HalfVec>,)*
                    #( #prev_double_col_idents: #double_col_tys,)*
                    #( #prev_col_idents: #delta_col_tys,)*
                    #( #prev_delta_idents: #delta_col_tys,)*
                    rows: usize,
                }

                impl ::tsz_compress::prelude::TszCompressV2 for #compressor_ident {
                    type T = #ident;

                    /// Sets up two compression queues: one for delta compression and one for delta-delta compression,
                    /// along with their respective output buffers. Initializes counters for the number of column values
                    /// emitted during the delta and delta-delta compression processes.
                    fn new(prealloc_rows: usize) -> Self {
                        #compressor_ident {
                            #( #col_delta_comp_queue_idents: ::tsz_compress::prelude::CompressionQueue::<10>::new(),)*
                            #( #col_delta_delta_comp_queue_idents: ::tsz_compress::prelude::CompressionQueue::<2>::new(),)*
                            #( #col_delta_buf_idents: #col_delta_buf,)*
                            #( #col_delta_delta_buf_idents: #col_delta_delta_buf,)*
                            #( #prev_double_col_idents: 0,)*
                            #( #prev_col_idents: 0,)*
                            #( #prev_delta_idents: 0,)*
                            rows: 0,
                        }
                    }

                    /// Performs compression using delta/delta-delta compression.
                    #[inline(always)]
                    fn compress(&mut self, row: Self::T) {
                        // Enqueues delta and delta-delta values
                        self.rows += 1;

                        if self.rows > 2 {
                            #(
                                // The new delta  and delta-delta
                                let col = row.#col_idents as #delta_col_tys;
                                let delta = col - self.#prev_col_idents;

                                // Do delta compression if configured
                                #delta_comp_block

                                // Do delta-delta compression if configured
                                #delta_delta_comp_block

                                // Update the previous values
                                self.#prev_col_idents = col;
                                self.#prev_delta_idents = delta;
                            )*
                        } else if self.rows == 1 {
                            /// Write out the full value in the exact bit-width of the column.
                            #(
                                if let Some(outbuf) = self.#col_delta_buf_idents.as_mut() {
                                    outbuf.push(::tsz_compress::prelude::halfvec::HalfWord::Half(::tsz_compress::prelude::consts::headers::START_OF_COLUMN));
                                    outbuf.push(::tsz_compress::prelude::halfvec::HalfWord::Half(::tsz_compress::prelude::consts::headers::FIRST_ROW));
                                    #write_first(outbuf, row.#col_idents);
                                }
                                if let Some(outbuf) = self.#col_delta_delta_buf_idents.as_mut() {
                                    outbuf.push(::tsz_compress::prelude::halfvec::HalfWord::Half(::tsz_compress::prelude::consts::headers::START_OF_COLUMN));
                                    outbuf.push(::tsz_compress::prelude::halfvec::HalfWord::Half(::tsz_compress::prelude::consts::headers::FIRST_ROW));
                                    #write_first(outbuf, row.#col_idents);
                                }
                                self.#prev_double_col_idents = row.#col_idents as #double_col_tys;
                            )*
                        } else if self.rows == 2 {
                            /// Write out the full value in the next exact bit-width of the column, regardless of chosen delta bit-width.
                            /// SAFETY: If the bit-width is configurable, then bits at rest will be uninterpretable.
                            #(
                                // Up cast to double bit-width always for the first delta
                                let col = row.#col_idents as #double_col_tys;
                                let delta = col - self.#prev_double_col_idents;
                                if let Some(outbuf) = self.#col_delta_buf_idents.as_mut() {
                                    outbuf.push(::tsz_compress::prelude::halfvec::HalfWord::Half(::tsz_compress::prelude::consts::headers::SECOND_ROW));
                                    #write_second(outbuf, delta);
                                }
                                if let Some(outbuf) = self.#col_delta_delta_buf_idents.as_mut() {
                                    outbuf.push(::tsz_compress::prelude::halfvec::HalfWord::Half(::tsz_compress::prelude::consts::headers::SECOND_ROW));
                                    #write_second(outbuf, delta);
                                }

                                // Use choice of bit-width for delta/delta-delta compression
                                self.#prev_delta_idents = delta as #delta_col_tys;
                                self.#prev_col_idents = col as #delta_col_tys;
                            )*;
                        }
                    }


                    fn len(&self) -> usize {
                        let mut finished_nibble_count = 0;
                        #(
                            if let (Some(delta_buffer), Some(delta_delta_buffer)) = (&self.#col_delta_buf_idents, &self.#col_delta_delta_buf_idents) {
                                finished_nibble_count += delta_buffer.len().min(delta_delta_buffer.len());
                            }
                            else if let Some(delta_buffer) = &self.#col_delta_buf_idents {
                                finished_nibble_count += delta_buffer.len()
                            }
                            else if let Some(delta_delta_buffer) = &self.#col_delta_delta_buf_idents {
                                finished_nibble_count += delta_delta_buffer.len()
                            }
                        )*
                        let col_count_delta = (#( self.#col_delta_comp_queue_idents.len() )+*);
                        let col_count_delta_delta = (#( self.#col_delta_delta_comp_queue_idents.len() )+*);
                        let col_bit_rate = #num_columns * self.bit_rate();
                        let pending_bit_count = col_count_delta.min(col_count_delta_delta) * col_bit_rate;
                        4 * finished_nibble_count + pending_bit_count
                    }

                    fn bit_rate(&self) -> usize {
                        let mut finished_nibble_count = 0;
                        let mut total_col_values_emitted = 0;
                        #(
                            if let (Some(delta_buffer), Some(delta_delta_buffer)) = (&self.#col_delta_buf_idents, &self.#col_delta_delta_buf_idents) {
                                finished_nibble_count += delta_buffer.len().min(delta_delta_buffer.len());
                            }
                            else if let Some(delta_buffer) = &self.#col_delta_buf_idents {
                                    finished_nibble_count += delta_buffer.len()
                                }
                            else if let Some(delta_delta_buffer) = &self.#col_delta_delta_buf_idents {
                                finished_nibble_count += delta_delta_buffer.len()
                            }
                        )*
                        if self.rows == 0 {
                            return 0;
                        }
                        4 * finished_nibble_count / self.rows
                    }

                    ///
                    /// The number of rows that have been compressed.
                    /// This is an exact answer for rows consumed including rows that may not have been emitted.
                    ///
                    #[inline(always)]
                    fn row_count(&self) -> usize {
                        self.rows
                    }

                    ///
                    /// Consumes the compressor state, appending compressed bytes
                    /// to the provided buffer and reserving space if needed.
                    ///
                    /// Leaving the intermediate buffers in a reserved, cleared state.
                    ///
                    fn finish_into(&mut self, output_bytes: &mut Vec<u8>) {
                        // Only use one encoding mechanism
                        #(
                            if let (Some(delta_buffer), Some(delta_delta_buffer)) = (&self.#col_delta_buf_idents, &self.#col_delta_delta_buf_idents) {
                                // Prefer delta on ties
                                if delta_delta_buffer.len() >= delta_buffer.len() {
                                    self.#col_delta_delta_buf_idents = None;
                                } else {
                                    self.#col_delta_buf_idents = None;
                                }
                            }
                        )*

                        // Guarantee that at least the column start nibble is emitted
                        #(
                            if let Some(outbuf) = self.#col_delta_buf_idents.as_mut() {
                                if outbuf.is_empty() {
                                    outbuf.push(::tsz_compress::prelude::halfvec::HalfWord::Half(::tsz_compress::prelude::consts::headers::START_OF_COLUMN));
                                }
                            }
                            if let Some(outbuf) = self.#col_delta_delta_buf_idents.as_mut() {
                                if outbuf.is_empty() {
                                    outbuf.push(::tsz_compress::prelude::halfvec::HalfWord::Half(::tsz_compress::prelude::consts::headers::START_OF_COLUMN));
                                }
                            }
                        )*

                        // Flush any pending samples in the queues
                        // All of the bits are concatenated with a 1001 tag indicating the start of a new column
                        #(
                            self.#col_delta_buf_idents.as_mut().map(|outbuf| {
                                while self.#col_delta_comp_queue_idents.len() > 0 {
                                    self.#col_delta_comp_queue_idents.flush_delta_bits(outbuf);
                                }
                             });
                            self.#col_delta_delta_buf_idents.as_mut().map(|outbuf| {
                                while self.#col_delta_delta_comp_queue_idents.len() > 0 {
                                    self.#col_delta_delta_comp_queue_idents.emit_delta_delta_bits(outbuf);
                                }
                            });
                        )*

                        // Write the number of rows as a 32-bit integer
                        // The decompressor will read this value and reserve space for the rows
                        // SAFETY: The number of rows may be more than 2^32, but the decompressor will
                        //         reserve at most 2^32 rows.
                        let mut rows = ::tsz_compress::prelude::halfvec::HalfVec::new(8);
                        ::tsz_compress::prelude::write_i32_bits(&mut rows, self.rows as u32 as i32);

                        // Create an iterator over the words to be written
                        let rows = Some(rows);
                        let words = [
                            rows.as_ref().into_iter(),
                            #(
                                self.#col_delta_buf_idents.as_ref().into_iter(),
                                self.#col_delta_delta_buf_idents.as_ref().into_iter(),
                            )*
                        ].into_iter().flatten();

                        // Pack the words into nibbles
                        ::tsz_compress::prelude::halfvec::HalfVec::finish(output_bytes, words);

                        // Clear the buffers for re-use
                        #(
                            self.#col_delta_buf_idents.as_mut().map(|outbuf| {
                                outbuf.clear();
                            });
                            self.#col_delta_delta_buf_idents.as_mut().map(|outbuf| {
                                outbuf.clear();
                            });
                            self.rows = 0;
                        )*
                    }

                    #finish_into_thin
                }
            }

            pub use private::#compressor_ident;
        }
    };

    compressor_struct.into()
}

#[proc_macro_derive(DecompressV2)]
pub fn derive_decompressv2(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as syn::DeriveInput);

    // We will define a struct by this name
    let ident = input.ident.clone();
    let decompressor_ident = format_ident!("{}DecompressorImpl", ident);

    let columns = get_fields_of_struct(input);
    let (col_idents, col_tys, _col_attrs): (Vec<_>, Vec<_>, Vec<_>) = multiunzip(columns);

    let col_vec_idents = col_idents
        .iter()
        .map(|ident| format_ident!("col_{}", ident))
        .collect_vec();

    let decode_idents = col_tys
        .iter()
        .map(|ty| match ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let segment = path.segments.first().unwrap();
                let ident = segment.ident.clone();
                match ident.to_string().as_str() {
                    "i8" => quote! { decode_i8 },
                    "i16" => quote! { decode_i16 },
                    "i32" => quote! { decode_i32 },
                    "i64" => quote! { decode_i64 },
                    _ => panic!("Unsupported type"),
                }
            }
            _ => panic!("Unsupported type"),
        })
        .collect::<Vec<_>>();

    let decompressor_tokens = quote! {
        pub mod decompress {
            use super::*;
            mod private {
                use super::*;
                use ::tsz_compress::prelude::*;

                /// A Decompressor type implementing TszDecompressV2.
                #[derive(Debug)]
                pub struct #decompressor_ident {
                    #( #col_vec_idents: Vec<#col_tys>, )*
                }

                impl #decompressor_ident {
                    #(
                        /// Decompressed values for the column
                        pub fn #col_vec_idents(&self) -> &[#col_tys] {
                            &self.#col_vec_idents
                        }
                    )*
                }

                impl ::tsz_compress::prelude::TszDecompressV2 for #decompressor_ident {
                    type T = #ident;

                    /// Initialize a decompressor with a vector for each column.
                    fn new() -> Self {
                        #decompressor_ident {
                            #( #col_vec_idents: Vec::new(), )*
                        }
                    }

                    /// Decompress tsz-compressed bytes, extending the columns with the decompressed values.
                    fn decompress(&mut self, bytes: &[u8]) -> Result<(), CodingError> {
                        // Require at least the row count and 1 column
                        if bytes.len() < core::mem::size_of::<i32>() + 1 {
                            return Err(CodingError::Empty);
                        }

                        // Read the row count, accepting a reservation up to 2^32 rows
                        // SAFETY: The decompressor will reserve at most 2^32 rows, but there may be more if overflow occurs.
                        let row_bytes: &[u8; 4] = bytes[..4].try_into().map_err(|_|CodingError::NotEnoughBits)?;
                        let rows = read_full_i32(row_bytes) as u32;
                        let bytes = &bytes[core::mem::size_of::<i32>()..];

                        // At best we can emit 3 bits per row not counting any metadata for one column
                        if rows as usize > bytes.len() * 8 / 3 {
                            return Err(CodingError::InvalidRowCount(rows as usize));
                        }

                        // Reserve space for the rows if there is enough remaining capacity
                        #(
                            let remaining = (self.#col_vec_idents.capacity() - self.#col_vec_idents.len()) as isize;
                            let reservation = rows as isize - remaining;
                            if reservation > 0 {
                                self.#col_vec_idents.reserve(reservation as usize);
                            }
                        )*

                        // Iterate over the bits
                        let mut iter = HalfIter::new(bytes);

                        // Expect a headers::START_OF_COLUMN tag indicating the start of a new column
                        if iter.next() != Some(::tsz_compress::prelude::consts::headers::START_OF_COLUMN) {
                            #( self.#col_vec_idents.clear(); )*
                            return Err(CodingError::InvalidInitialColumnTag);
                        }

                        // Read the column bytes into a vector one after the other
                        #( #decode_idents(&mut iter, &mut self.#col_vec_idents)?; )*

                        // Pad nibbles to byte-alignment
                        match iter.next() {
                            Some(::tsz_compress::prelude::consts::headers::START_OF_COLUMN) | None => (),
                            Some(_) => return Err(CodingError::InvalidColumnTag),
                        }

                        // Make sure all the columns are the same length
                        let elems = [ #( self.#col_vec_idents.len(), )* ];
                        if !elems.iter().all(|elem| *elem == elems[0]) {
                            #( self.#col_vec_idents.clear(); )*
                            return Err(CodingError::ColumnLengthMismatch(ColumnLengths { expected_rows: rows as usize, column_lengths: elems.to_vec() }));
                        }

                        Ok(())
                    }

                    /// Rotate the columns into rows
                    fn rows(&self) -> Vec<Self::T> {
                        // Create the rows from columns
                        let elems = [ #( self.#col_vec_idents.len(), )* ];
                        let len = elems[0];
                        let mut rows = Vec::with_capacity(len);
                        for i in 0..len {
                            rows.push(#ident {
                                #( #col_idents: unsafe { *self.#col_vec_idents.get_unchecked(i) }, )*
                            });
                        }
                        rows
                    }

                    /// Clear the internal state
                    fn clear(&mut self) {
                        #( self.#col_vec_idents.clear(); )*
                    }
                }
            }
            pub use private::#decompressor_ident;
        }

    };
    decompressor_tokens.into()
}
