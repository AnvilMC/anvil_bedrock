extern crate proc_macro;
use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ExprLit, Fields, ItemEnum, ItemStruct, Lit};

#[proc_macro_derive(MCPEPacketDataAuto)]
pub fn derive_answer_fn(stream: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(stream as ItemStruct);
    match parsed.fields {
        Fields::Named(e) => {
            let out = format!(
                r#"impl crate::traits::MCPEPacketData for {} {{
    fn decode(reader: &mut impl crate::traits::Reader) -> Result<Self, crate::prelude::MCPEPacketDataError> {{
        use crate::traits::PacketReader;
        Ok(Self{{{}}})
    }}

    fn encode(&self, writer: &mut impl crate::traits::Writer) -> Result<(), crate::prelude::MCPEPacketDataError> {{
        {}
        Ok(())
    }}
}}"#,
                parsed.ident.to_string(),
                e.named
                    .iter()
                    .map(|i| format!(
                        "{}: reader.auto_decode().map_err(|x| x.map(\"{}\"))?",
                        i.ident.as_ref().unwrap().to_string(),
                        i.ident.as_ref().unwrap().to_string()
                    ))
                    .collect::<Vec<_>>()
                    .join(", "),
                e.named
                    .iter()
                    .map(|i| format!(
                        "self.{}.encode(writer).map_err(|x| x.map(\"{}\"))?;",
                        i.ident.as_ref().unwrap().to_string(),
                        i.ident.as_ref().unwrap().to_string()
                    ))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            out.parse().unwrap()
        }
        Fields::Unnamed(e) => {
            let out = format!(
                r#"impl crate::traits::MCPEPacketData for {} {{
                    fn decode(reader: &mut impl crate::traits::Reader) -> Result<Self, crate::prelude::MCPEPacketDataError> {{
                        use crate::traits::PacketReader;
                        Ok(Self({}))
                    }}
                
                    fn encode(&self, writer: &mut impl crate::traits::Writer) -> Result<(), crate::prelude::MCPEPacketDataError> {{
                        {}
                        Ok(())
                    }}
                }}"#,
                parsed.ident.to_string(),
                e.unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format!("reader.auto_decode().map_err(|x| x.map(\"{}\"))?", i))
                    .collect::<Vec<_>>()
                    .join(", "),
                e.unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format!(
                        "self.{}.encode(writer).map_err(|x| x.map(\"{}\"))?;",
                        i, i
                    ))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            out.parse().unwrap()
        }
        Fields::Unit => unreachable!(),
    }
}

fn parse_hex(i: &str) -> u8 {
    fn h2b(i: u8) -> u8 {
        match i {
            b'a'..=b'f' => i - b'a' + 10,
            b'A'..=b'F' => i - b'A' + 10,
            b'0'..=b'9' => i - b'0',
            _ => unreachable!(),
        }
    }
    let b = i.as_bytes();
    h2b(b[0]) * 16 + h2b(b[1])
}

#[proc_macro_attribute]
pub fn packet(args: TokenStream, item: TokenStream) -> TokenStream {
    let i = args.into_iter().next().unwrap().to_string();
    let i = if i.starts_with("0x") {
        parse_hex(&i[2..])
    } else {
        i.parse::<u8>().unwrap()
    };

    let parsed = parse_macro_input!(item as ItemStruct);
    let ty = parsed.ident.clone();

    let token_stream = quote! {
        #parsed
        impl crate::traits::MCPEPacket for #ty {
            const PACKET_ID: u8 = #i;
        }
    };
    token_stream.into()
}

#[proc_macro_attribute]
pub fn mcpe_packet_data_enum(args: TokenStream, item: TokenStream) -> TokenStream {
    let i = args.into_iter().next().unwrap(); // .to_string()
    println!("{:?}", i);

    let parsed = parse_macro_input!(item as ItemEnum);
    //let variants = parsed.variants.clone();
    println!("{:?}", parsed.variants);
    let data_map: HashMap<String, i32> = parsed
        .variants
        .iter()
        .map(|x| {
            let name = x.ident.to_string();
            let discriminant = x.discriminant.as_ref().unwrap();
            if let Expr::Lit(ExprLit {
                lit: Lit::Int(a), ..
            }) = &discriminant.1
            {
                (name, a.to_string().parse().unwrap())
            } else {
                panic!("Should be int in enum discriminant")
            }
        })
        .collect();

    let imple = format!(
        r#"impl crate::traits::MCPEPacketData for {} {{
            fn decode(reader: &mut impl crate::traits::Reader) -> Result<Self, crate::prelude::MCPEPacketDataError> {{
                use crate::traits::PacketReader;
                Ok(match <{}>::decode(reader)? {{
                    {}
                    e => return Err(crate::prelude::MCPEPacketDataError::new("enum_ident", format!("Invalid enum identifier: {{}}", e)))
                }})
            }}
        
            fn encode(&self, writer: &mut impl crate::traits::Writer) -> Result<(), crate::prelude::MCPEPacketDataError> {{
                let ty: {} = match self {{
                    {}
                }};
                ty.encode(writer)?;
                Ok(())
            }}
        }}"#,
        parsed.ident.to_string(),
        i,
        data_map
            .iter()
            .map(|(x, y)| format!("{} => Self::{},", y, x))
            .collect::<Vec<_>>()
            .join("\n"),
        i,
        data_map
            .iter()
            .map(|(x, y)| format!("Self::{} => {},", x, y))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    let tokens: proc_macro2::TokenStream = imple.parse().unwrap();
    let token_stream = quote! {
        #parsed
        #tokens
    };
    token_stream.into()
}
