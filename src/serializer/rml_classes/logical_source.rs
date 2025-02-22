use crate::serializer::rml_classes::ReferenceFormulation;

#[derive(Debug)]
pub struct LogicalSource {
    iterator: String,
    reference_formulation: ReferenceFormulation,
    source: String,
}