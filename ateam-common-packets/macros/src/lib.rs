use proc_macro::TokenStream;
use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    Fields, Ident, Type,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct WireBitfieldArgs {
    base_type: Ident,
}

impl Parse for WireBitfieldArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(WireBitfieldArgs { base_type: input.parse()? })
    }
}

fn get_bits_attr(attrs: &[syn::Attribute]) -> Option<u32> {
    for attr in attrs {
        if attr.path().is_ident("bits") {
            if let Ok(n) = attr.parse_args::<syn::LitInt>() {
                return n.base10_parse().ok();
            }
        }
    }
    None
}

fn type_name(ty: &Type) -> Option<String> {
    if let Type::Path(tp) = ty {
        tp.path.segments.last().map(|s| s.ident.to_string())
    } else {
        None
    }
}

fn default_bits(type_str: &str) -> u32 {
    match type_str {
        "bool" => 1,
        "u8" | "i8" => 8,
        "u16" | "i16" => 16,
        "u32" | "i32" => 32,
        "u64" | "i64" => 64,
        _ => 0,
    }
}

/// Attribute macro that transforms a bitfield struct definition into a
/// `#[repr(transparent)]` newtype over the specified integer, with `const fn`
/// getter/builder/setter methods generated for every named (non-`_`-prefixed) field.
///
/// # Syntax
/// ```ignore
/// #[wire_bitfield(u32)]
/// pub struct Flags {
///     pub flag_a: bool,
///     pub flag_b: bool,
///     #[bits(6)]
///     pub count: u8,
///     #[bits(24)]
///     _reserved: u32,
/// }
/// ```
///
/// Fields are packed LSB-first in declaration order. `bool` fields are 1 bit.
/// Non-bool fields must carry `#[bits(N)]` or use their natural width (u8=8, etc.).
/// Reserved fields (`_`-prefixed) consume bits without generating accessors.
/// Total bits across all fields must equal the base type's bit width.
#[proc_macro_attribute]
pub fn wire_bitfield(args: TokenStream, input: TokenStream) -> TokenStream {
    let WireBitfieldArgs { base_type } = parse_macro_input!(args as WireBitfieldArgs);
    let ast = parse_macro_input!(input as syn::ItemStruct);

    let struct_name = &ast.ident;
    let vis = &ast.vis;

    let (total_bits, base_ty_tokens): (u32, TokenStream2) = match base_type.to_string().as_str() {
        "u8"  => (8,  quote! { u8 }),
        "u16" => (16, quote! { u16 }),
        "u32" => (32, quote! { u32 }),
        "u64" => (64, quote! { u64 }),
        other => {
            return syn::Error::new(
                base_type.span(),
                format!("wire_bitfield: unsupported base type `{}`; use u8/u16/u32/u64", other),
            )
            .to_compile_error()
            .into();
        }
    };

    let fields = match &ast.fields {
        Fields::Named(f) => f,
        _ => {
            return syn::Error::new(struct_name.span(), "wire_bitfield requires a named-field struct")
                .to_compile_error()
                .into();
        }
    };

    struct FieldEntry {
        name: Ident,
        bits: u32,
        reserved: bool,
        ty: Type,
    }

    let mut entries: Vec<FieldEntry> = Vec::new();

    for field in &fields.named {
        let name = field.ident.as_ref().unwrap().clone();
        let reserved = name.to_string().starts_with('_');
        let tname = type_name(&field.ty).unwrap_or_default();
        let bits = get_bits_attr(&field.attrs).unwrap_or_else(|| default_bits(&tname));

        if bits == 0 {
            return syn::Error::new(
                name.span(),
                format!(
                    "wire_bitfield: cannot determine bit width for field `{}`; add `#[bits(N)]`",
                    name
                ),
            )
            .to_compile_error()
            .into();
        }

        entries.push(FieldEntry { name, bits, reserved, ty: field.ty.clone() });
    }

    let used: u32 = entries.iter().map(|e| e.bits).sum();
    if used != total_bits {
        return syn::Error::new(
            struct_name.span(),
            format!(
                "wire_bitfield: fields sum to {} bits but `{}` is {} bits",
                used, base_type, total_bits
            ),
        )
        .to_compile_error()
        .into();
    }

    let mut methods = TokenStream2::new();
    let mut bit_offset: u32 = 0;

    for entry in &entries {
        let offset = bit_offset;
        bit_offset += entry.bits;

        if entry.reserved {
            continue;
        }

        let getter = &entry.name;
        let setter = Ident::new(&format!("set_{}", entry.name), entry.name.span());
        let builder = Ident::new(&format!("with_{}", entry.name), entry.name.span());

        let tname = type_name(&entry.ty).unwrap_or_default();
        let off_lit = Literal::u32_unsuffixed(offset);

        if tname == "bool" {
            methods.extend(quote! {
                #[inline(always)]
                pub const fn #getter(self) -> bool {
                    (self.0 >> #off_lit) & 1 != 0
                }
                #[inline(always)]
                pub const fn #builder(self, v: bool) -> Self {
                    Self((self.0 & !(1 << #off_lit)) | ((v as #base_ty_tokens) << #off_lit))
                }
                #[inline(always)]
                pub fn #setter(&mut self, v: bool) {
                    *self = self.#builder(v);
                }
            });
        } else {
            let field_ty = &entry.ty;
            let bits = entry.bits;
            // mask fits in u64 for all supported base types
            let mask = (1u64 << bits) - 1;
            let mask_lit = Literal::u64_unsuffixed(mask);

            methods.extend(quote! {
                #[inline(always)]
                pub const fn #getter(self) -> #field_ty {
                    ((self.0 >> #off_lit) & (#mask_lit as #base_ty_tokens)) as #field_ty
                }
                #[inline(always)]
                pub const fn #builder(self, v: #field_ty) -> Self {
                    let mask = (#mask_lit as #base_ty_tokens) << #off_lit;
                    Self((self.0 & !mask) | (((v as #base_ty_tokens) & #mask_lit as #base_ty_tokens) << #off_lit))
                }
                #[inline(always)]
                pub fn #setter(&mut self, v: #field_ty) {
                    *self = self.#builder(v);
                }
            });
        }
    }

    let byte_count = Literal::usize_unsuffixed((total_bits / 8) as usize);

    quote! {
        #[repr(transparent)]
        #[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
        #vis struct #struct_name(pub #base_ty_tokens);

        impl #struct_name {
            #methods
        }

        const _: () = ::core::assert!(
            ::core::mem::size_of::<#struct_name>() == #byte_count
        );
    }
    .into()
}
