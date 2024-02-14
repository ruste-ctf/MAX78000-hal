use std::ops::Bound;

use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parenthesized,
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    token::{Bracket, Comma, Paren},
    Attribute, DeriveInput, Expr, ExprLit, ExprRange, Ident, Item, ItemMod, ItemStruct, Lit,
    LitInt, Meta, MetaNameValue, Path, RangeLimits, Token,
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
                    Expr::Lit(ExprLit { attrs: _, lit }) => match lit {
                        Lit::Int(int) => Ok(Bound::Included(int.base10_parse()?)),
                        _ => Err(input.error("Require literal int in range")),
                    },
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
                    Expr::Lit(ExprLit { attrs: _, lit }) => match lit {
                        Lit::Int(int) => {
                            let value = int.base10_parse()?;

                            if second_is_included {
                                Ok(Bound::Included(value))
                            } else {
                                Ok(Bound::Excluded(value))
                            }
                        }
                        _ => Err(input.error("Require literal int in range")),
                    },
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

                doc_attr.push(string.value().trim_start().into());
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

    let register_names: Vec<String> = parsed_scope
        .bits
        .iter()
        .map(|bits| bits.bit_attr.register_name.clone())
        .collect();

    let register_paths: Vec<Path> = parsed_scope
        .bits
        .iter()
        .map(|bits| bits.bit_attr.path.clone())
        .collect();

    let registers_struct = generate_reg_struct(&register_paths, &register_names);
    let bit_impl: Vec<proc_macro2::TokenStream> = parsed_scope
        .bits
        .iter()
        .map(|bit_item| generate_bit(bit_item))
        .collect();

    let emit = quote! {
        #registers_struct

        impl Registers {
            #(#bit_impl)*
        }
    };

    emit.into()
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

fn generate_range((start, end): (usize, usize)) -> proc_macro2::TokenStream {
    quote!(
        #start ..= #end
    )
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
    let doc_string = string_into_title(name);
    let doc_example_let = format!("let my_const = Registers::{};", name_const);
    let doc_example_assert = format!("assert_eq!(my_const, {});", value);
    quote!(
        #doc_string
        #docs
        ///
        /// # Const Item
        /// This is a const item that defines where this `bit`'s ranges are. This range
        /// can simply be a single bit, or multiple bits depending on the bit
        /// type presented. If the bits for this flag are defined with a range (ie. `0..=7`)
        /// then two const items will be made with the names `<MY FLAG>_BIT_START` and
        /// `<MY FLAG>_BIT_END`. In the above example, the `<MY FLAG>_BIT_START` will
        /// contain the value '0', and `<MY FLAG>_BIT_END` will be '7'.
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

fn generate_range_get(name: String, (start, end): (usize, usize)) -> proc_macro2::TokenStream {
    let name = format_ident!("{}", name.to_lowercase().replace(" ", "_"));
    let bit_type = min_type_for_range((start, end));
    quote! {
        pub fn #name() -> #bit_type {
            0
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

    let const_start = generate_const(
        &format!("{}_BIT_START", bit.name),
        start,
        doc_string.clone(),
    );
    let const_end = generate_const(&format!("{}_BIT_END", bit.name), end, doc_string.clone());
    let getter = generate_range_get(format!("get_{}", bit.name), (start, end));

    quote!(
        #const_start
        #const_end

        #getter

    )
}

fn generate_bit_single(single: usize, bit: &BitBlock) -> proc_macro2::TokenStream {
    quote!()
}

fn generate_reg_struct(
    all_register_offsets: &Vec<Path>,
    all_register_names: &Vec<String>,
) -> proc_macro2::TokenStream {
    assert_eq!(all_register_offsets.len(), all_register_names.len());

    let reg_names: Vec<_> = all_register_names
        .iter()
        .map(|value| format_ident!("{value}"))
        .collect();

    quote! {
        #[repr(C)]
        #[allow(unused)]
        pub struct Registers {
           #( #reg_names: u32, )*
        }
    }
}

// Struct Output:
// #[repr(C)]
// struct PrvReg {
//    tmr_cnt: ReadWriteCell<u32>
//    tmr_cmp: ReadWriteCell<u32>
//    tmr_pwm: ReadWriteCell<u32>
//    tmr_intfl: ReadWriteCell<u32>
// }
// pub struct Registers(inner: PrvReg);
//
// impl Registers {
//    pub fn new(port: usize) -> Result<Self> {
//       assert!(port == DINGUS1 || port == DINGUS2 ...);
//       Self(PrvReg::new(port))
//    }
//
//    #[inline(always)]
//    pub fn tmr_cnt() -> u32 {self.0.tmr_cnt.read()}
//    pub unsafe fn set_tmr_cnt(value: u32) {self.0.tmr_cnt.write(value)}
//
//    pub fn timer_compare_value(&self) -> u32 {
//       self.tmr_cnt().get_bit_range(0..=31)
//    }
// }

/*
    /// let mut reg = Registers::new(mmio::TIMER_0).unwrap();
    ///
    /// reg.set_time_count(32)
    ///
    ///
    /// struct Timer {
    ///     reg: registers::Register
    /// }
    ///
    /// impl Timer {
    ///     fn test(&self) {
    ///        self.reg.set_time_count(10)
    ///     }
    /// }
    make_device! {
        #[device_ports(mmio::TIMER_0, mmio::TIMER_1, mmio::TIMER_2)]

        /// Set the count of the timer.
        #[bit(0..=31, RW, rro::TMR_CNT)]
        time_count

        ///
        /// get_time_count -> u32
        /// set_time_count(u32)
        /// TIME_COUNT_START_BIT: usize = 0
        /// TIME_COUNT_END_BIT: usize = 31

        /// The timer compare value.
        #[bit(0..=31, RW, rro::TMR_CMP)]
        timer_compare_value

        /// The timer PWM register.
        #[bit(0..=31, RW, rro::TMR_PWM)]
        pwm

        /// The timer Interrupt register.
        #[bit(25, RO, rro::TMR_INTFL)]
        timerb_write_done

        //example of some RW1C
        #[bit(13, RW1C, rro::DINGUS)]
        done_flag

        /// Maybe Nice to have
        #[bit(24, RW, rro::TMR_DINGUS)]
        #[restrict(write > 0 && write <= 8)] // This could be something later
        timerb_write_fifo

        /// ...
    }

    /// Debug Output:
    /// Registers {
    ///    time_count: 0xFF
    ///    timer_compare_value: 0xAA
    ///    pwm: 0x00
    ///    timerb_write_done: false
    ///    done_flag: NotActive
    /// }
    ///
    ///
    ///
*/
