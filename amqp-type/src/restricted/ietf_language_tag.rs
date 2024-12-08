use std::vec::IntoIter;
use crate::error::amqp_error::AmqpError;
use crate::error::AppError;
use crate::primitive::Primitive;
use crate::primitive::variable_width::symbol::Symbol;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};

/// # IETF Language Tag
///
/// An IETF language tag as defined by BCP 47.
/// ##### AMQP Spec
/// ```xml
/// <type name="ietf-language-tag" class="restricted" source="symbol"/>
/// ```
/// IETF language tags are abbreviated language codes as defined in the IETF Best Current Practice
/// [BCP-47](http://www.rfc-editor.org/rfc/bcp/bcp47.txt)
/// (incorporating [RFC-5646](http://www.rfc-editor.org/rfc/rfc5646.txt)).
/// A list of registered subtags is maintained in the
/// [IANA Language Subtag Registry](http://www.iana.org/assignments/language-subtag-registry).
///
/// All AMQP implementations should understand at the least the IETF language tag en-US (note that
/// this uses a hyphen separator, not an underscore).
#[derive(Debug, Clone, PartialEq)]
pub struct IetfLanguageTag(Symbol);

impl IetfLanguageTag {
    pub fn new(value: String) -> Self {
        match VALID_LANGUAGE_CODES.contains(&value.as_str()) {
            true => {
                match Symbol::new(value) {
                    Ok(valid) => Self(valid),
                    Err(_) => {
                        Self::default()
                    }
                }
            }
            false => Self::default()
        }
    }

    pub fn inner(&self) -> &str {
        self.0.inner()
    }

    pub fn into_inner(self) -> Symbol {
        self.0
    }
}

impl Encode for IetfLanguageTag {
    fn encode(self) -> Encoded {
        self.0.encode()
    }
}

impl Decode for IetfLanguageTag {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized
    {
        Symbol::try_decode(constructor, stream).map(Self)
    }
}

impl Default for IetfLanguageTag {
    fn default() -> Self {
        Self(Symbol::with_ascii("en-us"))
    }
}

impl From<IetfLanguageTag> for Primitive {
    fn from(value: IetfLanguageTag) -> Self {
        Primitive::Symbol(value.0)
    }
}

impl TryFrom<Primitive> for IetfLanguageTag {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Symbol(s) => {
                Ok(IetfLanguageTag(s))
            }
            _ => Err(AmqpError::DecodeError)?
        }
    }
}

const VALID_LANGUAGE_CODES: [&str; 284] = [
    "aa", "ab", "ae", "af", "ak", "am", "an", "ar", "ar-ae", "ar-bh", "ar-dz", "ar-eg", "ar-iq",
    "ar-jo", "ar-kw", "ar-lb", "ar-ly", "ar-ma", "ar-om", "ar-qa", "ar-sa", "ar-sy", "ar-tn",
    "ar-ye", "as", "av", "ay", "az", "ba", "be", "bg", "bh", "bi", "bm", "bn", "bo", "br", "bs",
    "ca", "ce", "ch", "co", "cr", "cs", "cu", "cv", "cy", "da", "de", "de-at", "de-ch", "de-de",
    "de-li", "de-lu", "div", "dv", "dz", "ee", "el", "en", "en-au", "en-bz", "en-ca", "en-cb",
    "en-gb", "en-ie", "en-jm", "en-nz", "en-ph", "en-tt", "en-us", "en-za", "en-zw", "eo", "es",
    "es-ar", "es-bo", "es-cl", "es-co", "es-cr", "es-do", "es-ec", "es-es", "es-gt", "es-hn",
    "es-mx", "es-ni", "es-pa", "es-pe", "es-pr", "es-py", "es-sv", "es-us", "es-uy", "es-ve", "et",
    "eu", "fa", "ff", "fi", "fj", "fo", "fr", "fr-be", "fr-ca", "fr-ch", "fr-fr", "fr-lu", "fr-mc",
    "fy", "ga", "gd", "gl", "gn", "gu", "gv", "ha", "he", "hi", "ho", "hr", "hr-ba", "hr-hr", "ht",
    "hu", "hy", "hz", "ia", "id", "ie", "ig", "ii", "ik", "in", "io", "is", "it", "it-ch", "it-it",
    "iu", "iw", "ja", "ji", "jv", "jw", "ka", "kg", "ki", "kj", "kk", "kl", "km", "kn", "ko",
    "kok", "kr", "ks", "ku", "kv", "kw", "ky", "kz", "la", "lb", "lg", "li", "ln", "lo", "ls",
    "lt", "lu", "lv", "mg", "mh", "mi", "mk", "ml", "mn", "mo", "mr", "ms", "ms-bn", "ms-my", "mt",
    "my", "na", "nb", "nd", "ne", "ng", "nl", "nl-be", "nl-nl", "nn", "no", "nr", "ns", "nv", "ny",
    "oc", "oj", "om", "or", "os", "pa", "pi", "pl", "ps", "pt", "pt-br", "pt-pt", "qu", "qu-bo",
    "qu-ec", "qu-pe", "rm", "rn", "ro", "ru", "rw", "sa", "sb", "sc", "sd", "se", "se-fi", "se-no",
    "se-se", "sg", "sh", "si", "sk", "sl", "sm", "sn", "so", "sq", "sr", "sr-ba", "sr-sp", "ss",
    "st", "su", "sv", "sv-fi", "sv-se", "sw", "sx", "syr", "ta", "te", "tg", "th", "ti", "tk",
    "tl", "tn", "to", "tr", "ts", "tt", "tw", "ty", "ug", "uk", "ur", "us", "uz", "ve", "vi", "vo",
    "wa", "wo", "xh", "yi", "yo", "za", "zh", "zh-cn", "zh-hk", "zh-mo", "zh-sg", "zh-tw", "zu",
];


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ietf_language_tag_should_contain_only_valid_ietf_codes() {
        assert_eq!("de-at", IetfLanguageTag::new("de-at".into()).inner());
    }

    #[test]
    fn test_ietf_language_tag_falls_back_to_en_us_as_default() {
        assert_eq!("en-us", IetfLanguageTag::new("invalid".into()).inner());
    }

}