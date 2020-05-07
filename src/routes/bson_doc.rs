use serde::Serialize;

pub(crate) trait BsonDoc: Serialize {
    fn as_bson(&self) -> bson::EncoderResult<bson::ordered::OrderedDocument> {
        if let bson::Bson::Document(document) = bson::to_bson(self)? {
            Ok(document)
        } else {
            unreachable!("Type should be convertible to document.");
        }
    }
}
