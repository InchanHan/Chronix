use proc_macro::TokenStream;
use quote::quote;
use syn::{AttributeArgs, ItemFn, Lit, Meta, NestedMeta, parse_macro_input};

#[proc_macro_attribute]
pub fn chronixer(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let args = parse_macro_input!(attr as AttributeArgs);

    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let vis = &input.vis;
    let sig = &input.sig;

    let mut warmup: u32 = 1;
    let mut reps: u32 = 10;
    let mut aggregation: String = String::from("min");
    let mut pin_core0: bool = false;
    let mut accesses: u32 = 0;
    let mut cpu_ghz: f64 = 0.0;
    for arg in &args {
        if let NestedMeta::Meta(Meta::NameValue(m)) = arg {
            let ident = m.path.get_ident().unwrap().to_string();
            match ident.as_str() {
                "warmup" => {
                    if let Lit::Int(lit_int) = &m.lit {
                        warmup = lit_int.base10_parse::<u32>().unwrap();
                    }
                }
                "reps" => {
                    if let Lit::Int(lit_int) = &m.lit {
                        reps = lit_int.base10_parse::<u32>().unwrap();
                    }
                }
                "agg" => {
                    if let Lit::Str(lit_str) = &m.lit {
                        aggregation = lit_str.value();
                    }
                }
                "pin" => {
                    if let Lit::Bool(lit_bool) = &m.lit {
                        pin_core0 = lit_bool.value();
                    }
                }
                // cpu_ghz and accesses are used only for rdtscp timer
                // user must type accesses manually
                "cpu_ghz" => {
                    if let Lit::Float(lit_float) = &m.lit {
                        cpu_ghz = lit_float.base10_parse::<f64>().unwrap();
                    }
                }
                "accesses" => {
                    if let Lit::Int(lit_int) = &m.lit {
                        accesses = lit_int.base10_parse::<u32>().unwrap();
                    }
                }
                _ => {}
            }
        }
    }

    if (accesses == 0) || (cpu_ghz == 0.0) {
        #[cfg(feature = "rdtscp")]
        {
            return syn::Error::new_spanned(
                &args[0],
                "rdtscp timer requires 'accesses' and 'cpu_ghz' parameter!!",
            )
            .to_compile_error()
            .into();
        }
    }

    let expanded = quote! {
        #vis #sig {
            let agg = match #aggregation {
                "Median" => chronix::runner::Aggregation::Median,
                "P95" => chronix::runner::Aggregation::P95,
                _ => chronix::runner::Aggregation::Min,
            };

            #[cfg(feature = "rdtscp")]
            let timer = chronix::timer::RdtscpTimer { cpu_ghz: #cpu_ghz };

            #[cfg(not(feature = "rdtscp"))]
            let timer = chronix::timer::InstantTimer;

            let cfg = chronix::runner::BenchConfig::new(#warmup, #reps, agg, #pin_core0);
            let mut run = chronix::runner::Runner::new(timer, cfg.clone());
            let (stat, result) = run.measure(|| {#fn_block}, 10);
            let stdout_struct = chronix::output::Stdout;
            chronix::output::Sink::report(&stdout_struct, stringify!(#fn_name), &stat, &cfg);
            result
        }
    };

    expanded.into()
}
