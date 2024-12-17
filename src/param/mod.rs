mod general;

use self::general::GeneralKeywords;

#[derive(Debug, Clone)]
pub struct CastepParam {
    general_keywords: GeneralKeywords,
}
