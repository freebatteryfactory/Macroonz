#![forbid(unsafe_code)]

use std::io::{Read, Write};

bakery::recipe! {
    pub mod codec {
        #[derive(Debug, PartialEq, Eq)]
        pub struct Ledger { pub count: u16 }
        impl Ledger {
            pub const fn assembled(count: u16) -> Self { Self { count } }
        }
        bake! {
            codecs {
                ledger(Ledger) {
                    direction(round_trip);
                    refusal(LedgerDecodeError);
                    assembly(assembled, total);
                    members { count: u16 => count(required); };
                };
            };
            projections { codec; };
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut input = Vec::new();
    std::io::stdin().take(64).read_to_end(&mut input)?;
    let decoded = std::hint::black_box(codec::Ledger::decode_canonical(&input));
    writeln!(std::io::stdout(), "{decoded:?}")
}
