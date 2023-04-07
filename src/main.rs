use std::net::ToSocketAddrs;
use std::simd::{i64x4, i64x8};
use std::sync::Barrier;

use std::vec::Vec;




enum ArrowTypes {
    Int64,
    Int32,
    Int16,
    Int8,
    UInt64,
    UInt32,
    UInt16,
    UInt8,
    Float64,
    Float32,
    Date32,
    Date64,
    Timestamp,
    Time32,
    Time64,
    Interval,
    Duration,
    Binary,
    Utf8,
    Bool,
    Decimal,
    List,
    Struct,
    Union,
    Dictionary,
    FixedSizeBinary,
    FixedSizeList,
    Null,
    Map,
    Custom,
}


struct Field {
    name: String,
    data_type: ArrowTypes,
    nullable: bool,
    children: Vec<Field>,
    dictionary: Option<Box<Field>>,
    custom_metadata: Option<Box<Field>>,
    metadata: Option<Box<Field>>,
}

struct Schema {
    fields: Vec<Field>,
    metadata: Option<Box<Field>>,
}

struct ArrayData {
    len: usize,
    null_count: usize,
    offset: usize,
    buffers: Vec<Vec<u8>>,
    child_data: Vec<ArrayData>,
    dictionary: Option<Box<ArrayData>>,
    custom_metadata: Option<Box<ArrayData>>,
    metadata: Option<Box<ArrayData>>,
}

struct Array {
    data: ArrayData,
    offset: usize,
}

struct RecordBatch {
    schema: Schema,
    columns: Vec<Array>,
}

struct RecordBatchReader {
    schema: Schema,
    columns: Vec<Array>,
}

struct RecordBatchWriter {
    schema: Schema,
    columns: Vec<Array>,
}

struct RecordBatchStream {
    schema: Schema,
    columns: Vec<Array>,
}

struct RecordBatchFile {
    schema: Schema,
    columns: Vec<Array>,
}

struct RecordBatchFileReader {
    schema: Schema,
    columns: Vec<Array>,
}

struct RecordBatchFileWriter {
    schema: Schema,
    columns: Vec<Array>,
}

struct RecordBatchFileAppender {
    schema: Schema,
    columns: Vec<Array>,
}


struct Arrow<R,T> {

    // An optional name
    name: Option<String>::None,

    ret_type: R,

    // Argument input types, need a way to specify each type uniquely
    inp_args: Vec<Field>,
    arg_types: Vec<ArrowTypes>,

    arity: u8,

    // The steps to execute
    func: Vec<fn(&[ArrowTypes]) -> ArrowTypes>,


    // an array of strings with length equal to the value of arity
    // each string is the name of the input
    inp_names: Vec<String>,

    // an array of strings with length equal to the value of arity
    // each string is the name of the output
    out_names: Vec<String>,




}




struct MathematicalCategory {
    name: String,
    description: String,
    subcategories: Vec<MathematicalCategory>,

    /* Objects */
    objects: Vec<MathematicalObject>,

    /* Morphisms */
    morphisms: Vec<MathematicalMorphism>,
}


fn main() {
    println!("Hello, world!");
}
