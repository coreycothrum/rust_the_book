enum Language {
    English(String),
    PigLatin(String),
}

impl Language {
    // fn to_english(self) -> Self {
    //     return Self::English(match self {
    //         Self::English(e) => { e }
    //         Self::PigLatin(pl) => { pl } //FIXME
    //     })
    // }

    fn to_pig_latin(self) -> Self {
        return Self::PigLatin(match self {
            Self::English(e) => {
                let mut pl = String::new();

                let mut chars = e.chars().peekable();

                while let Some(c) = chars.next() {
                    let end = match c {
                        'a' | 'e' | 'i' | 'o' | 'u' => {
                            pl.push(c);
                            String::from("hay")
                        }
                        'a'..='z' | 'A'..='Z' => {
                            format!("{}ay", c)
                        }
                        _ => {
                            pl.push(c);
                            continue;
                        }
                    };

                    while let Some(&c) = chars.peek() {
                        match c {
                            'a'..='z' | 'A'..='Z' => {
                                chars.next();
                                pl.push(c);
                            }
                            _ => {
                                break;
                            }
                        }
                    }

                    pl += &end;
                }
                pl
            }
            Self::PigLatin(pl) => pl,
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_pig_latin() {
        if let Language::PigLatin(pl) = Language::English(String::from("apple")).to_pig_latin() {
            assert_eq!(pl, String::from("applehay"));
        }
        if let Language::PigLatin(pl) = Language::English(String::from("truck")).to_pig_latin() {
            assert_eq!(pl, String::from("rucktay"));
        }
    }
}
