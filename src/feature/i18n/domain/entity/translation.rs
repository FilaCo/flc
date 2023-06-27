use api::::domain::vo::{TranslationId, TranslationNode};

pub struct Translation<'a> {
    id: TranslationId<'a>,
    ru_ru: TranslationNode<'a>,
    en_us: TranslationNode<'a>,
}
