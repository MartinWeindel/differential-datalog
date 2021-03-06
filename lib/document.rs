use unstructured;
use unstructured::Document;
use differential_datalog::record::*;
use std::collections::BTreeMap;

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

pub fn document_document2string(doc: &document_Document) -> String {
    doc.x.to_string()
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


#[cfg(feature = "flatbuf")]
impl<FB> FromFlatBuffer<FB> for document_Document
{
    fn from_flatbuf(fb: FB) -> Response<Self> {
        Ok(serde_json::from_str::<Document>(String::from_flatbuf(fb)?))
    }
}

#[cfg(feature = "flatbuf")]
impl<'b> ToFlatBuffer<'b> for document_Document
{
    type Target = <String as ToFlatBuffer<'b>>::Target;

    fn to_flatbuf(&self, fbb: &mut fbrt::FlatBufferBuilder<'b>) -> Self::Target {
        serde_json::to_string(&self.x).unwrap_or_default("{}").to_flatbuf(fbb)
    }
}

#[cfg(feature = "flatbuf")]
impl<'b> ToFlatBufferTable<'b> for document_Document
{
    type Target = <String as ToFlatBufferTable<'b>>::Target;

    fn to_flatbuf_table(
        &self,
        fbb: &mut fbrt::FlatBufferBuilder<'b>,
    ) -> fbrt::WIPOffset<Self::Target> {
        serde_json::to_string(&self.x).unwrap_or_default("{}").to_flatbuf_table(fbb)
    }
}

#[cfg(feature = "flatbuf")]
impl<'b, T> ToFlatBufferVectorElement<'b> for document_Document
{
    type Target = <String as ToFlatBufferVectorElement<'b>>::Target;

    fn to_flatbuf_vector_element(&self, fbb: &mut fbrt::FlatBufferBuilder<'b>) -> Self::Target {
        serde_json::to_string(&self.x).unwrap_or_default("{}").to_flatbuf_vector_element(fbb)
    }
}

pub fn document_doc_unit() -> document_Document {
    document_Document {x: Document::Unit}
}

pub fn document_doc_from_json_string(
    json: &String,
) -> std_Result<document_Document, String> {
    res2std(serde_json::from_str::<Document>(json).map(|d| document_Document {x:d}))
}

pub fn document_doc_to_json_string(doc: &document_Document) -> std_Result<String, String> {
    res2std(serde_json::to_string(&doc.x))
}

pub fn document_doc_from_yaml_string(
    s: &String,
) -> std_Result<document_Document, String> {
    res2std(serde_yaml::from_str::<Document>(s).map(|d| document_Document {x:d}))
}

pub fn document_doc_to_yaml_string(doc: &document_Document) -> std_Result<String, String> {
    res2std(serde_yaml::to_string(&doc.x))
}

pub fn document_doc_select<'a>(
    doc: &'a document_Document,
    sel: &'a String,
) -> std_Result<document_Document, String> {
    res2std(doc.x.select(sel).map(|d| document_Document {x:(*d).clone()}))
}

pub fn document_doc_is_string(doc: &document_Document) -> bool {
    doc.x.is_string()
}

pub fn document_doc_is_map(doc: &document_Document) -> bool {
    doc.x.is_map()
}

pub fn document_doc_is_vec(doc: &document_Document) -> bool {
    doc.x.is_seq()
}

pub fn document_doc_is_number(doc: &document_Document) -> bool {
    doc.x.is_number()
}

pub fn document_doc_is_s64(doc: &document_Document) -> bool {
    doc.x.is_i64()
}

pub fn document_doc_is_u64(doc: &document_Document) -> bool {
    doc.x.is_u64()
}

pub fn document_doc_is_unit(doc: &document_Document) -> bool {
    doc.x.is_unit()
}

pub fn document_doc_is_bool(doc: &document_Document) -> bool {
    doc.x.is_bool()
}

pub fn document_doc_as_string(doc : &document_Document) -> std_Option<String> {
    option2std(doc.x.as_string())
}

pub fn document_doc_as_map(doc : &document_Document) -> std_Option<std_Map<document_Document,document_Document>> {
    option2std(doc.x.as_map().map(|map| {
        let mut res = std_Map::new();

        for (key, value) in map.iter() {
            res.insert(document_Document {x:key.clone()}, document_Document {x:value.clone()});
        }
        return res;
    }))
}

pub fn document_doc_as_vec(doc : &document_Document) -> std_Option<std_Vec<document_Document>> {
    option2std(doc.x.as_seq().map(|seq| {
        let mut res = std_Vec::new();
        res.x.reserve(seq.len());

        for value in seq.iter() {
            res.push(document_Document {x:value.clone()});
        }
        return res;
    }))
}

pub fn document_doc_as_s64(doc : &document_Document) -> std_Option<i64> {
    option2std(doc.x.as_i64())
}

pub fn document_doc_as_u64(doc : &document_Document) -> std_Option<u64> {
    option2std(doc.x.as_u64())
}

pub fn document_doc_as_bool(doc : &document_Document) -> std_Option<bool> {
    option2std(doc.x.as_bool())
}

pub fn document_doc_from_bool(b: bool) -> document_Document {
    document_Document {x: Document::from(b)}
}

pub fn document_doc_from_string(s: String) -> document_Document {
    document_Document {x: Document::from(s)}
}

pub fn document_doc_from_u64(u: u64) -> document_Document {
    document_Document {x: Document::from(u)}
}

pub fn document_doc_from_s64(s: i64) -> document_Document {
    document_Document {x: Document::from(s)}
}

pub fn document_doc_from_vec(seq: &std_Vec<document_Document>) -> document_Document {
    let mut res = Vec::new();
    res.reserve(seq.len());

    for value in seq.iter() {
        res.push(value.x.clone());
    }

    document_Document {x: Document::from(res)}
}

pub fn document_doc_from_map(map: &std_Map<String, document_Document>) -> document_Document {
    let mut res = BTreeMap::new();

    for (key, value) in map.iter() {
        res.insert(Document::from(key), value.x.clone());
    }
    return document_Document {x: Document::from(res)};
}
