use strum::Display;
use strum::EnumString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, EnumString, Eq, PartialEq, Serialize, Deserialize)]
#[strum(serialize_all = "SCREAMING-KEBAB-CASE")]
pub enum Language{
    Bh,
    Cz,
    Da,
    De,
    El,
    EnGb,
    EnUs,
    Es,
    Et,
    Fi,
    Fr,
    Hu,
    Id,
    It,
    Ja,
    Ko,
    Lt,
    Lv,
    Nb,
    Nl,
    Pl,
    PtBr,
    PtPt,
    Ro,
    Ru,
    Sk,
    Sl,
    Sv,
    Tr,
    Uk,
    Zh
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Translation{
    detected_source_language: Language,
    text: String
}

///
/// Deepl translation api response
/// # Example
/// ```
/// {
///   "translations": [
///    {
///       "detected_source_language": "EN",
///       "text": "Hallo, Welt!"
///     }
///   ]
/// }
/// ```
///
#[derive(Debug, Serialize, Deserialize)]
pub struct DeeplAnswer {
    translations: Vec<Translation>
}




