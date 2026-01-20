enum Currency {
    Kopeyka,
    Ruble,
    Sotochka,
    Kosarik,
    Pyatyora(Version),
}

#[derive(Debug)]
enum Version {
    Old,
    New,
}

enum Variant {
    One,
    Two,
    Three,
    Four,
}

fn main() {
    fn get_currency_name(currency: Option<Currency>) -> String {
        match currency {
            None => "None".to_string(),
            Some(cur) => match cur {
                Currency::Kopeyka => "Kopeyka".to_string(),
                Currency::Ruble => "Ruble".to_string(),
                Currency::Sotochka => "Sotochka".to_string(),
                Currency::Kosarik => "Kosarik".to_string(),
                Currency::Pyatyora(version) => {
                    println!("Version is {version:?}, купить можно что-нибудь");
                    "Pyatyora".to_string()
                }
            },
        }
    }
    println!(
        "{}",
        get_currency_name(Some(Currency::Pyatyora(Version::New)))
    );
    println!(
        "{}",
        get_currency_name(Some(Currency::Pyatyora(Version::Old)))
    );
    println!("{}", get_currency_name(Some(Currency::Kopeyka)));
    println!("{}", get_currency_name(None));

    fn match_variant(variant: Variant) {
        match variant {
            Variant::One => println!("One"),
            Variant::Two => println!("Two"),
            _ => println!("Three or Four"),
        }
    }

    match_variant(Variant::One);
    match_variant(Variant::Two);
    match_variant(Variant::Three);
    match_variant(Variant::Four);

    let variant = Variant::Three;
    if let Variant::Three = variant {
        println!("Three if let");
    }

    let variant = Some(Variant::Four);
    if let Some(Variant::Four) = variant {
        println!("Four if let");
    } else {
        println!("Not Four");
    }
}
