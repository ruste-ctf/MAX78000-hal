use std::ops::Bound;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::{Bracket, Comma, Paren},
    Attribute, DeriveInput, Expr, ExprRange, Ident, Item, ItemMod, LitInt, Meta, Token,
};

#[derive(Debug)]
enum BitRange {
    Range(ExprRange),
    Single(LitInt),
}

impl Parse for BitRange {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(range) = input.parse() {
            Ok(Self::Range(range))
        } else if let Ok(int) = input.parse() {
            Ok(Self::Single(int))
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
}

impl Parse for Access {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(access::RW) {
            input.parse::<access::RW>()?;
            Ok(Access::RW)
        } else {
            todo!("Imp more Access -> {:#?}", input.to_string())
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

struct BitBlock {
    doc_attr: Vec<Attribute>,
    name: Ident,
}

impl Parse for BitBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let mut doc_attr: Vec<Attribute> = Vec::new();

        for attr in &attributes {
            if attr.path().is_ident("doc") {
                doc_attr.push(attr.clone());
            }
            if attr.path().is_ident("bit") {
                let bit: BitAttribute = attr.parse_args()?;
                todo!("{:#?}", bit);
            }
        }
        Ok(Self {
            doc_attr: attributes,
            name: input.parse()?,
        })
    }
}

struct MakeDevice {
    bits: Punctuated<BitBlock, Token![,]>,
}

impl Parse for MakeDevice {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(MakeDevice {
            bits: input.parse_terminated(BitBlock::parse, Token![,])?,
        })
    }
}

#[proc_macro]
pub fn make_device(input: TokenStream) -> TokenStream {
    let thing = parse_macro_input!(input as MakeDevice);

    todo!("END OF MACRO")
}

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
    /// Struct Output:
    /// #[repr(C)]
    /// struct PrvReg {
    ///    tmr_cnt: ReadWriteCell<u32>
    ///    tmr_cmp: ReadWriteCell<u32>
    ///    tmr_pwm: ReadWriteCell<u32>
    ///    tmr_intfl: ReadWriteCell<u32>
    /// }
    /// pub struct Registers(inner: PrvReg);
    ///
    /// impl Registers {
    ///    pub fn new(port: usize) -> Result<Self> {
    ///       assert!(port == DINGUS1 || port == DINGUS2 ...);
    ///       Self(PrvReg::new(port))
    ///    }
    ///
    ///    #[inline(always)]
    ///    pub fn tmr_cnt() -> u32 {self.0.tmr_cnt.read()}
    ///    pub unsafe fn set_tmr_cnt(value: u32) {self.0.tmr_cnt.write(value)}
    ///
    ///    pub fn timer_compare_value(&self) -> u32 {
    ///       self.tmr_cnt().get_bit_range(0..=31)
    ///    }
    /// }
    ///
*/
