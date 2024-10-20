use crate::primitive::variable_width::symbol::Symbol;

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
/// A list of registered subtags is maintained in the [IANA Language Subtag Registry](http://www.iana.org/assignments/language-subtag-registry).
///
/// All AMQP implementations should understand at the least the IETF language tag en-US (note that
/// this uses a hyphen separator, not an underscore).
#[derive(Debug, Clone)]
pub struct IetfLanguageTag(Symbol);
