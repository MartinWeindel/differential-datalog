use unstructured::Document;
use differential_datalog::record::*;

#[derive(Eq, PartialOrd, PartialEq, Ord, Clone, Hash, Serialize, Deserialize, Default)]
pub struct document_Document {
    pub x: Document
}

impl fmt::Display for document_Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.x)
    }
}

impl fmt::Debug for document_Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl FromRecord for document_Document {
    fn from_record(val: &Record) -> Result<Self, String> {
        match val {
            Record::String(s) => match serde_json::from_str::<Document>(&s) {
                Result::Ok(d) => Result::Ok(document_Document { x: d }),
                Result::Err(e) => Result::Err(format!("{}", e)), 
            },
            v => Result::Err(format!("not a string {:?}", *v)),
        }
    }
}

impl IntoRecord for document_Document {
    fn into_record(self) -> Record {
        Record::String(serde_json::to_string(&self.x).unwrap())
    }
}

impl Mutator<document_Document> for Record {
    fn mutate(&self, doc: &mut document_Document) -> Result<(), String> {
        document_Document::from_record(self).map(|d| doc.x = d.x)
    }
}

pub fn document_unit() -> document_Document {
    document_Document {x: Document::Unit}
}

pub fn document_document_from_json_string<'a>(
    json: &'a String,
) -> std_Result<document_Document, String> {
    res2std(serde_json::from_str::<'a, Document>(json).map(|d| document_Document {x:d}))
}

pub fn document_document_to_json_string(doc: &document_Document) -> std_Result<String, String> {
    res2std(serde_json::to_string(&doc.x))
}

pub fn document_select<'a>(
    doc: &'a document_Document,
    sel: &'a String,
) -> std_Result<document_Document, String> {
    res2std(doc.x.select(sel).map(|d| document_Document {x:(*d).clone()}))
}
