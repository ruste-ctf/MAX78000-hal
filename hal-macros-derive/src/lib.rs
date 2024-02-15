use std::{collections::HashMap, ops::Bound};

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::{Comma, Paren},
    Attribute, Expr, ExprLit, ExprRange, Ident, Lit, LitInt, Meta, MetaNameValue, Path,
    RangeLimits, Token,
};

#[derive(Debug)]
enum BitRange {
    Range((Bound<usize>, Bound<usize>)),
    Single(usize),
}

impl Parse for BitRange {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if (input.peek(LitInt) || input.peek(Token![..]))
            && (input.peek2(Token![..]) || input.peek2(LitInt))
        {
            let range: ExprRange = input.parse()?;
            let first = range
                .start
                .map(|expr| match expr.as_ref() {
                    Expr::Lit(ExprLit {
                        attrs: _,
                        lit: Lit::Int(int),
                    }) => Ok(Bound::Included(int.base10_parse()?)),
                    _ => Err(input.error("Require literal int in range")),
                })
                .unwrap_or(Ok(Bound::Unbounded))?;

            let second_is_included = match range.limits {
                RangeLimits::HalfOpen(_) => false,
                RangeLimits::Closed(_) => true,
            };

            let second = range
                .end
                .map(|expr| match expr.as_ref() {
                    Expr::Lit(ExprLit {
                        attrs: _,
                        lit: Lit::Int(int),
                    }) => {
                        let value = int.base10_parse()?;

                        if second_is_included {
                            Ok(Bound::Included(value))
                        } else {
                            Ok(Bound::Excluded(value))
                        }
                    }
                    _ => Err(input.error("Require literal int in range")),
                })
                .unwrap_or(Ok(Bound::Unbounded))?;

            Ok(Self::Range((first, second)))
        } else if input.peek(LitInt) {
            let value: LitInt = input.parse()?;

            Ok(Self::Single(value.base10_parse()?))
        } else {
            Err(input.error("Could not parse BitRange"))
        }
    }
}

#[derive(Debug)]
enum Access {
    RW,
    RO,
    WO,
    RW1C,
    RW1O,
}

mod access {
    syn::custom_keyword!(RW);
    syn::custom_keyword!(RO);
    syn::custom_keyword!(WO);
    syn::custom_keyword!(RW1C);
    syn::custom_keyword!(RW1O);
}

impl Parse for Access {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(access::RW) {
            input.parse::<access::RW>()?;
            Ok(Access::RW)
        } else if input.peek(access::RO) {
            input.parse::<access::RO>()?;
            Ok(Access::RO)
        } else if input.peek(access::WO) {
            input.parse::<access::WO>()?;
            Ok(Access::WO)
        } else if input.peek(access::RW1C) {
            input.parse::<access::RW1C>()?;
            Ok(Access::RW1C)
        } else if input.peek(access::RW1O) {
            input.parse::<access::RW1O>()?;
            Ok(Access::RW1O)
        } else {
            Err(input.error("Not a valid access token"))
        }
    }
}

#[derive(Debug)]
struct BitAttribute {
    bit: BitRange,
    access: Access,
    path: syn::Path,
    register_name: String,
}

impl Parse for BitAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let bit = input.parse()?;
        input.parse::<Comma>()?;
        let access = input.parse()?;
        input.parse::<Comma>()?;
        let path: syn::Path = input.parse()?;
        let register_name = path
            .segments
            .last()
            .ok_or(input.error("Could not find const path ident"))?
            .ident
            .to_string()
            .to_ascii_lowercase();

        Ok(Self {
            bit,
            access,
            path,
            register_name,
        })
    }
}

#[derive(Debug)]
struct BitBlock {
    doc_attr: Vec<String>,
    bit_attr: BitAttribute,
    name: Ident,
}

impl Parse for BitBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let mut doc_attr: Vec<String> = Vec::new();
        let mut bit_attr: Option<BitAttribute> = None;

        for attr in &attributes {
            if attr.path().is_ident("doc") {
                let Meta::NameValue(MetaNameValue {
                    value:
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(string),
                            ..
                        }),
                    ..
                }) = &attr.meta
                else {
                    return Err(input.error("Could not parse doc comment"));
                };

                doc_attr.push(format!(" {}", string.value().trim_start()));
            } else if attr.path().is_ident("bit") {
                bit_attr = Some(attr.parse_args()?);
            } else {
                return Err(input.error("Unknown attribute"));
            }
        }

        Ok(Self {
            doc_attr,
            bit_attr: bit_attr.ok_or(input.error("Reqires a #[bit(...)]"))?,
            name: input.parse()?,
        })
    }
}

#[derive(Debug)]
struct MakeDevice {
    device_ports: DevicePorts,
    bits: Vec<BitBlock>,
}

#[derive(Debug)]
struct DevicePorts(Vec<Path>);

mod device_ports {
    syn::custom_keyword!(device_ports);
}

impl Parse for DevicePorts {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<device_ports::device_ports>()?;
        let content;
        let _: Paren = parenthesized!(content in input);
        input.parse::<Token![;]>()?;
        Ok(Self(
            content
                .parse_terminated(Path::parse, Comma)?
                .into_iter()
                .collect(),
        ))
    }
}

impl Parse for MakeDevice {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(MakeDevice {
            device_ports: input.parse()?,
            bits: input
                .parse_terminated(BitBlock::parse, Token![,])?
                .into_iter()
                .collect(),
        })
    }
}

#[proc_macro]
pub fn make_device(input: TokenStream) -> TokenStream {
    let parsed_scope = parse_macro_input!(input as MakeDevice);

    let register_names: Vec<(String, Path)> = parsed_scope
        .bits
        .iter()
        .map(|bits| {
            (
                bits.bit_attr.register_name.clone(),
                bits.bit_attr.path.clone(),
            )
        })
        .collect();

    let register_fields = generate_reg_fields(&register_names);
    let registers_struct = generate_reg_struct(&register_fields);
    let bit_impl: Vec<proc_macro2::TokenStream> = parsed_scope
        .bits
        .iter()
        .map(|bit_item| generate_bit(bit_item))
        .collect();

    let set_masks = generate_set_masks(&parsed_scope.bits);
    let new_fn = generate_new_constructer(&register_fields, parsed_scope.device_ports);

    let emit = quote! {
        #registers_struct

        impl Registers {
            #new_fn

            #set_masks
            #(#bit_impl)*
        }
    };

    emit.into()
}

fn generate_new_constructer(
    register_fields: &[(Ident, Path)],
    device_ports: DevicePorts,
) -> proc_macro2::TokenStream {
    let device_ports_vec = device_ports.0;
    let device_ports_string: String = device_ports_vec
        .iter()
        .map(|path_token| format!(", {}", quote!(#path_token).to_string().replace(" ", "")))
        .collect::<String>()
        .chars()
        .skip(2)
        .collect();

    let fields: Vec<_> = register_fields
        .iter()
        .map(|(ident, _)| quote!(#ident : RW::new(port).unwrap()))
        .collect();

    quote!(
        pub fn new(port: usize) -> Self {
            debug_assert!(
                false #( || #device_ports_vec == port)*,
                "Register port {port} must be {}", #device_ports_string
            );

            Self {
                #(#fields,)*
            }
        }
    )
}

fn generate_set_masks(bit: &Vec<BitBlock>) -> proc_macro2::TokenStream {
    let mut bit_map: HashMap<String, u32> = HashMap::new();

    for b in bit.iter() {
        let key = b.bit_attr.register_name.to_string();
        match b.bit_attr.access {
            Access::RW1C | Access::RW1O => {
                let bit_or_mask = {
                    match b.bit_attr.bit {
                        BitRange::Range(range) => {
                            let (start, end) = get_real_range(range);

                            let mut mask: u32 = 1;
                            for _ in 0..(end - start) {
                                mask <<= 1;
                                mask |= 1;
                            }

                            mask << start
                        }
                        BitRange::Single(single) => 1 << (single as u32),
                    }
                };

                if let Some(bit) = bit_map.get_mut(&key) {
                    *bit |= bit_or_mask;
                } else {
                    bit_map.insert(key, bit_or_mask);
                }
            }
            _ => {
                if !bit_map.contains_key(&key) {
                    bit_map.insert(key, 0);
                }
            }
        }
    }

    let set_mask_collection: Vec<proc_macro2::TokenStream> = bit_map
        .into_iter()
        .map(|(key, value)| {
            let const_name = format!("{key}_SET_MASK");
            generate_const(const_name.as_str(), !value as usize, quote!(#[doc(hidden)]))
        })
        .collect();

    let generating = quote!(
        #( #set_mask_collection )*
    );

    generating
}

fn generate_bit(bit: &BitBlock) -> proc_macro2::TokenStream {
    match bit.bit_attr.bit {
        BitRange::Range(range) => generate_bit_range(range, bit),
        BitRange::Single(single) => generate_bit_single(single, bit),
    }
}

fn get_real_range(range: (Bound<usize>, Bound<usize>)) -> (usize, usize) {
    let start = match range.0 {
        Bound::Unbounded => 0,
        Bound::Included(value) => value,
        Bound::Excluded(value) => value + 1,
    };
    let end = match range.1 {
        Bound::Unbounded => 31,
        Bound::Included(value) => value,
        Bound::Excluded(value) => value - 1,
    };

    (start, end)
}

fn string_into_title(name: &str) -> proc_macro2::TokenStream {
    let name = format!(
        " # {}",
        name.to_lowercase()
            .replace("_", " ")
            .chars()
            .into_iter()
            .fold(
                (true, String::new()),
                |(is_last_space, mut string), value| {
                    string.push(if is_last_space {
                        value.to_ascii_uppercase()
                    } else {
                        value
                    });
                    (value == ' ', string)
                },
            )
            .1
    );

    quote!(
        #[doc = #name]
    )
}

fn generate_const(
    name: &str,
    value: usize,
    docs: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let name_const = name.to_uppercase().replace(" ", "_");
    let name_tokens = format_ident!("{}", name_const);
    let doc_title = string_into_title(name);
    let doc_example_let = format!(" let my_const = Registers::{};", name_const);
    let doc_example_assert = format!(" assert_eq!(my_const, {});", value);
    quote!(
        #doc_title
        #docs
        ///
        /// # Const Item
        /// This is a const item that defines where this `bit`'s ranges are. This range
        /// can simply be a single bit, or multiple bits depending on the bit
        /// type presented. If the bits for this flag are defined with a range (ie. `0..=7`)
        /// then two const items will be made with the names `<MY FLAG>_BIT_START` and
        /// `<MY FLAG>_BIT_END`. In the above example, the `<MY FLAG>_BIT_START` will
        /// contain the value 0, and `<MY FLAG>_BIT_END` will be 7.
        ///
        /// # Example
        /// ```ignore
        #[doc = #doc_example_let]
        #[doc = #doc_example_assert]
        /// ```
        pub const #name_tokens: usize = #value;
    )
}

fn min_type_for_range((start, end): (usize, usize)) -> proc_macro2::TokenStream {
    let diff = end - start;

    match diff {
        ..=7 => quote!(u8),
        ..=15 => quote!(u16),
        ..=31 => quote!(u32),
        ..=63 => quote!(u64),
        _ => todo!("Out of range for bit range"),
    }
}

fn generate_range_get(
    name: &str,
    bit: &BitBlock,
    (start, end): (usize, usize),
) -> proc_macro2::TokenStream {
    let name = format_ident!("{}", name.to_lowercase().replace(" ", "_"));
    let bit_type = min_type_for_range((start, end));
    let self_dot = format_ident!("{}", bit.bit_attr.register_name);
    let const_name = bit.name.to_string().to_uppercase().replace(" ", "_");
    let self_mask = format_ident!("{}_BIT_MASK", const_name);
    let self_shift = format_ident!("{}_BIT_START", const_name);
    let doc_title = string_into_title(name.to_string().as_str());
    let doc = generate_doc_strings(&bit.doc_attr);
    quote! {
        #doc_title
        #doc
        ///
        /// # Get
        /// Gets the value or value range from the given register.
        ///
        /// # Safety
        /// It is ultimately up to the caller to ensure this function will
        /// never cause any side effects. However, usually reading from
        /// registers does not modify any processor state (just looks at it).
        ///
        /// # Volatile
        /// This function only preforms **1** volatile *read* and immediately copies
        /// the value and extracts the bits to return the result.
        ///
        #[inline(always)]
        pub fn #name(&self) -> #bit_type {
            use hal_macros::VolatileRead;
            (((self.#self_dot.read() as usize) & <Self>::#self_mask) >> <Self>::#self_shift) as #bit_type
        }
    }
}

fn generate_single_get(name: &str, bit: &BitBlock) -> proc_macro2::TokenStream {
    let name = format_ident!("{}", name.to_lowercase().replace(" ", "_"));
    let self_dot = format_ident!("{}", bit.bit_attr.register_name);
    let const_name = bit.name.to_string().to_uppercase().replace(" ", "_");
    let self_shift = format_ident!("{}_BIT", const_name);
    let doc_title = string_into_title(name.to_string().as_str());
    let doc = generate_doc_strings(&bit.doc_attr);
    quote! {
        #doc_title
        #doc
        ///
        /// # Get
        /// Gets the value or value range from the given register.
        ///
        /// # Safety
        /// It is ultimately up to the caller to ensure this function will
        /// never cause any side effects. However, usually reading from
        /// registers does not modify any processor state (just looks at it).
        ///
        /// # Volatile
        /// This function only preforms **1** volatile *read* and immediately copies
        /// the value and extracts the bits to return the result.
        ///
        #[inline(always)]
        pub fn #name(&self) -> bool {
            use hal_macros::VolatileRead;
            (self.#self_dot.read() & (<Self>::#self_shift as u32)) != 0
        }
    }
}

fn generate_single_set(name: &str, bit: &BitBlock, only_gen_one: bool) -> proc_macro2::TokenStream {
    let name = format_ident!("{}", name.to_lowercase().replace(" ", "_"));
    let self_dot = format_ident!("{}", bit.bit_attr.register_name);
    let const_name = bit.name.to_string().to_uppercase().replace(" ", "_");
    let self_shift = format_ident!("{}_BIT", const_name);
    let doc_title = string_into_title(name.to_string().as_str());
    let doc = generate_doc_strings(&bit.doc_attr);
    let reg_const_name = bit
        .bit_attr
        .register_name
        .to_string()
        .to_uppercase()
        .replace(" ", "_");
    let self_mask = format_ident!("{}_SET_MASK", reg_const_name);

    let param = if only_gen_one {
        quote!()
    } else {
        quote!(, flag: bool)
    };
    let flag_or_true = if only_gen_one {
        quote!(1)
    } else {
        quote!(if flag { 1 } else { 0 })
    };
    quote! {
        #doc_title
        #doc
        ///
        /// # Set
        /// Set the value or value range into the given register.
        ///
        /// # Safety
        /// It is up to the caller to verify that this register write will not
        /// cause any side effects. There could be an event that setting this
        /// register could cause undefined behavior elsewhere in the program.
        ///
        /// This register will deference the given `ptr` + `offset`, so one
        /// must verify at compile time that the given `ptr` falls within
        /// acceptable memory ranges.
        ///
        /// ## Other Register State
        /// In some examples it is true that ones register state depends on another
        /// register's status. In these cases, it is up to the caller to properly
        /// set this register to a valid (and ONLY valid value).
        ///
        /// # Volatile
        /// This function only preforms **1** volatile *read*,
        /// immediately modifies the flag and does **1** volatile *write* using
        /// the internal provided function to register.
        ///
        #[inline(always)]
        pub unsafe fn #name(&mut self #param) {
            use hal_macros::{VolatileRead, VolatileWrite};
            let read_value: u32 = self.#self_dot.read() & (<Self>::#self_mask as u32);
            let flag_value: u32 = (#flag_or_true) << (<Self>::#self_shift as u32);
            self.#self_dot.write(read_value | flag_value);
        }
    }
}

fn generate_range_set(
    name: &str,
    bit: &BitBlock,
    (start, end): (usize, usize),
) -> proc_macro2::TokenStream {
    let name = format_ident!("{}", name.to_lowercase().replace(" ", "_"));
    let bit_type = min_type_for_range((start, end));
    let self_dot = format_ident!("{}", bit.bit_attr.register_name);
    let const_name = bit.name.to_string().to_uppercase().replace(" ", "_");
    let self_mask = format_ident!("{}_BIT_MASK", const_name);
    let self_shift = format_ident!("{}_BIT_START", const_name);
    let const_reg_name = bit.bit_attr.register_name.to_uppercase().replace(" ", "_");
    let self_set_mask = format_ident!("{}_SET_MASK", const_reg_name);
    let doc_title = string_into_title(name.to_string().as_str());
    let doc = generate_doc_strings(&bit.doc_attr);
    quote! {
        #doc_title
        #doc
        ///
        /// # Set
        /// Set the value or value range into the given register.
        ///
        /// # Safety
        /// It is up to the caller to verify that this register write will not
        /// cause any side effects. There could be an event that setting this
        /// register could cause undefined behavior elsewhere in the program.
        ///
        /// This register will deference the given `ptr` + `offset`, so one
        /// must verify at compile time that the given `ptr` falls within
        /// acceptable memory ranges.
        ///
        /// ## Other Register State
        /// In some examples it is true that ones register state depends on another
        /// register's status. In these cases, it is up to the caller to properly
        /// set this register to a valid (and ONLY valid value).
        ///
        /// # Volatile
        /// This function only preforms **1** volatile *read*,
        /// immediately modifies the flag and does **1** volatile *write* using
        /// the internal provided function to register.
        ///
        /// # Panic
        /// This function will panic if provided flag value falls outside
        /// the given range of bits provided. **Assertions are only enabled during
        /// `debug` and will be disabled during release!** Please ensure provided
        /// input will only fall within valid acceptable range when setting this
        /// register.
        ///
        #[inline(always)]
        pub unsafe fn #name(&mut self, flag: #bit_type) {
            use hal_macros::{VolatileRead, VolatileWrite};
            debug_assert_eq!((flag as usize) >> (<Self>::#self_shift + 1), 0, "Provided flag {flag} is too large for provided setter range {}..={}!", #start, #end);
            let flag_shift: u32 = (flag as u32) << (<Self>::#self_shift as u32);
            let read_value: u32 = self.#self_dot.read() & (<Self>::#self_mask as u32) & (<Self>::#self_set_mask as u32);
            self.#self_dot.write(read_value | flag_shift);
        }
    }
}

fn generate_doc_strings(strings: &Vec<String>) -> proc_macro2::TokenStream {
    quote!(
        #(#[doc = #strings])*
    )
}

fn generate_bit_range(
    range: (Bound<usize>, Bound<usize>),
    bit: &BitBlock,
) -> proc_macro2::TokenStream {
    let (start, end) = get_real_range(range);

    let doc_string = generate_doc_strings(&bit.doc_attr);

    let mut mask: u32 = 1;
    for _ in 0..(end - start) {
        mask <<= 1;
        mask |= 1;
    }
    mask <<= start;

    let const_start = generate_const(
        &format!("{}_BIT_START", bit.name),
        start,
        doc_string.clone(),
    );
    let const_end = generate_const(&format!("{}_BIT_END", bit.name), end, doc_string.clone());

    let const_mask = generate_const(
        &format!("{}_BIT_MASK", bit.name),
        mask as usize,
        doc_string.clone(),
    );

    let (write, read) = match bit.bit_attr.access {
        Access::RO => (false, true),
        Access::WO => (true, false),
        _ => (true, true),
    };

    let getter = if read {
        generate_range_get(format!("get_{}", bit.name).as_str(), bit, (start, end))
    } else {
        quote!()
    };
    let setter = if write {
        generate_range_set(format!("set_{}", bit.name).as_str(), bit, (start, end))
    } else {
        quote!()
    };

    quote!(
        #const_start
        #const_end
        #const_mask

        #getter
        #setter
    )
}

fn generate_bit_single(single: usize, bit: &BitBlock) -> proc_macro2::TokenStream {
    let doc_string = generate_doc_strings(&bit.doc_attr);

    let (setter_name, setter_one, getter_start, getter_name) = match bit.bit_attr.access {
        Access::RW1C => ("clear", true, "is_", "_active"),
        Access::RW1O => ("activate", true, "is_", "_pending"),
        _ => ("set", false, "get_", ""),
    };

    let const_start = generate_const(
        format!("{}_BIT", bit.name).as_str(),
        1 << single,
        doc_string,
    );

    let (write, read) = match bit.bit_attr.access {
        Access::RO => (false, true),
        Access::WO => (true, false),
        _ => (true, true),
    };

    let getter = if read {
        generate_single_get(
            format!("{}{}{}", getter_start, bit.name, getter_name).as_str(),
            bit,
        )
    } else {
        quote!()
    };
    let setter = if write {
        generate_single_set(
            format!("{}_{}", setter_name, bit.name).as_str(),
            bit,
            setter_one,
        )
    } else {
        quote!()
    };
    quote!(
        #const_start

        #getter
        #setter
    )
}

fn generate_reg_fields(all_register_names: &[(String, Path)]) -> Vec<(Ident, Path)> {
    let mut register_map = HashMap::new();

    for (str, path) in all_register_names.iter() {
        if !register_map.contains_key(str) {
            register_map.insert(str, path);
        }
    }

    register_map
        .into_iter()
        .map(|(str, path)| (format_ident!("{}", str), path.clone()))
        .collect()
}

fn generate_reg_struct(reg_names: &[(Ident, Path)]) -> proc_macro2::TokenStream {
    let properties: Vec<proc_macro2::TokenStream> = reg_names
        .iter()
        .map(|(ident, path)| quote!(#ident: RW<{#path}, u32>))
        .collect();

    quote! {
        /// # Registers
        /// This struct was generated with the `make_device!` macro! This struct
        /// represents some hardware device expressed with `#[bit(...)]` attributes.
        ///
        /// # Example Of Registers
        /// ```rust
        /// use hal_macros_derive::make_device;
        /// use hal_macros::RW;
        ///
        /// const MY_DEVICE_PORT0: usize = 0xdeadbeef;
        /// const MY_DEVICE_PORT1: usize = 0xbadbabe3;
        ///
        /// const MY_REGISTER_OFFSET: usize = 0x0000;
        ///
        /// make_device! {
        ///    device_ports(MY_DEVICE_PORT0, MY_DEVICE_PORT1);
        ///
        ///    #[bit(0, RW, MY_REGISTER_OFFSET)]
        ///    my_reg_field,
        ///
        ///    #[bit(1, RO, MY_REGISTER_OFFSET)]
        ///    my_reg_other_read_only,
        ///
        ///    #[bit(2..=10, WO, MY_REGISTER_OFFSET)]
        ///    my_reg_range_write_only,
        /// }
        ///
        /// ```
        #[allow(unused)]
        pub struct Registers {
           #(#properties,)*
        }
    }
}
