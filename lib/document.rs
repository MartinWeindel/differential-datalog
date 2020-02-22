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

pub fn document_document_from_json_string(
    json: &String,
) -> std_Result<document_Document, String> {
    res2std(serde_json::from_str::<Document>(json).map(|d| document_Document {x:d}))
}

pub fn document_document_to_json_string(doc: &document_Document) -> std_Result<String, String> {
    res2std(serde_json::to_string(&doc.x))
}

pub fn document_document_from_yaml_string(
    s: &String,
) -> std_Result<document_Document, String> {
    res2std(serde_yaml::from_str::<Document>(s).map(|d| document_Document {x:d}))
}

pub fn document_document_to_yaml_string(doc: &document_Document) -> std_Result<String, String> {
    res2std(serde_yaml::to_string(&doc.x))
}

pub fn document_select<'a>(
    doc: &'a document_Document,
    sel: &'a String,
) -> std_Result<document_Document, String> {
    res2std(doc.x.select(sel).map(|d| document_Document {x:(*d).clone()}))
}

pub fn document_is_string(doc: &document_Document) -> bool {
    doc.x.is_string()
}

pub fn document_is_map(doc: &document_Document) -> bool {
    doc.x.is_map()
}

pub fn document_is_seq(doc: &document_Document) -> bool {
    doc.x.is_seq()
}

pub fn document_is_number(doc: &document_Document) -> bool {
    doc.x.is_number()
}

pub fn document_is_s64(doc: &document_Document) -> bool {
    doc.x.is_i64()
}

pub fn document_is_u64(doc: &document_Document) -> bool {
    doc.x.is_u64()
}

pub fn document_is_unit(doc: &document_Document) -> bool {
    doc.x.is_unit()
}

pub fn document_is_bool(doc: &document_Document) -> bool {
    doc.x.is_bool()
}

pub fn document_as_string(doc : &document_Document) -> std_Option<String> {
    option2std(doc.x.as_string())
}

pub fn document_as_map(doc : &document_Document) -> std_Option<std_Map<document_Document,document_Document>> {
    option2std(doc.x.as_map().map(|map| {
        let mut res = std_Map::new();

        for (key, value) in map.iter() {
            res.insert(document_Document {x:key.clone()}, document_Document {x:value.clone()});
        }
        return res;
    }))
}

pub fn document_as_seq(doc : &document_Document) -> std_Option<std_Vec<document_Document>> {
    option2std(doc.x.as_seq().map(|seq| {
        let mut res = std_Vec::new();
        res.x.reserve(seq.len());

        for value in seq.iter() {
            res.push(document_Document {x:value.clone()});
        }
        return res;
    }))
}

pub fn document_as_s64(doc : &document_Document) -> std_Option<i64> {
    option2std(doc.x.as_i64())
}

pub fn document_as_u64(doc : &document_Document) -> std_Option<u64> {
    option2std(doc.x.as_u64())
}

pub fn document_as_bool(doc : &document_Document) -> std_Option<bool> {
    option2std(doc.x.as_bool())
}