//

use azml::azml::id_num;

//region IntegerIdNumContext

#[derive(Clone, Gtmpl)]
pub struct IntegerIdNumContext {
    primitive_size: i8,
    primitive_size_bytes: i8,
    type_name: String,
    significant_bits: i8,
    significant_bits_mask: String,
    bitfield: IntegerIdNumBitfieldContext,
}

impl From<&id_num::IntegerIdNum> for IntegerIdNumContext {
    fn from(x: &id_num::IntegerIdNum) -> IntegerIdNumContext {
        IntegerIdNumContext {
            primitive_size: x.primitive_size(),
            primitive_size_bytes: x.primitive_size() / 8,
            type_name: format!("int{}", x.primitive_size()),
            significant_bits: x.significant_bits,
            significant_bits_mask: significant_bit_mask_bin(x.primitive_size(), x.significant_bits),
            // one for zero-based index, one for skipping the most-significant bit (the sign bit)
            bitfield: IntegerIdNumBitfieldContext::from(&x.bitfield, x.primitive_size() - 2, 0),
        }
    }
}

fn significant_bit_mask_bin(bit_size: i8, significant_size: i8) -> String {
    let mut v: u64 = 0;
    for i in 0..bit_size {
        if i < significant_size {
            v |= 1 << i;
        }
    }
    format_u64_as_bin(v)
}

fn format_u64_as_bin(i: u64) -> String {
    let mut s = String::new();
    let i_str = format!("{:b}", i);
    let a = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 8 == 0 {
            s.insert(0, '_');
        }
        s.insert(0, val);
    }
    format!("0b{}", s)
}

//endregion

//region IntegerIdNumBitfieldContext

#[derive(Clone, Gtmpl)]
pub struct IntegerIdNumBitfieldContext {
    pub sub_fields: Vec<IntegerIdNumBitfieldSubFieldContext>,
}

impl IntegerIdNumBitfieldContext {
    fn from(
        x: &id_num::IntegerIdNumBitfield,
        bitfield_size: i8,
        index_offset: i8,
    ) -> IntegerIdNumBitfieldContext {
        let mut all_fields: Vec<IntegerIdNumBitfieldSubFieldContext> = Vec::new();
        let mut idx: i8 = 0;
        for v in &x.sub_fields {
            let fields = convert_field(&v, "".to_owned(), bitfield_size, index_offset + idx, 0, 0);
            all_fields.extend(fields);
            idx += 1;
        }
        IntegerIdNumBitfieldContext {
            sub_fields: all_fields,
        }
    }
}

//endregion

//region IntegerIdNumBitfieldSubFieldContext

#[derive(Clone, Gtmpl)]
pub struct IntegerIdNumBitfieldSubFieldContext {
    identifier: String,
    doc_lines: Vec<String>,
    mask: String,
    flag: String,
}

fn convert_field(
    field: &id_num::IntegerIdNumBitfieldSubField,
    identifier_prefix: String,
    bitfield_size: i8,
    index_offset: i8,
    mask: u64,
    flag: u64,
) -> Vec<IntegerIdNumBitfieldSubFieldContext> {
    if !field.values.is_empty() {
        let mut all_fields: Vec<IntegerIdNumBitfieldSubFieldContext> = Vec::new();
        let mut flag: u64 = flag;
        for v in &field.values {
            let fields = convert_value(
                &v,
                identifier_prefix.to_owned(),
                bitfield_size,
                index_offset + field.size,
                mask | (1 << (bitfield_size - index_offset)),
                flag,
            );
            all_fields.extend(fields);
            flag |= 1 << (bitfield_size - index_offset);
        }
        all_fields
    } else {
        //TODO:
        // - ensure identifier is not empty
        // - return single
        let bit_offset = bitfield_size - index_offset;
        let mask = mask | (1 << bit_offset);
        let flag = flag | (1 << bit_offset);
        vec![IntegerIdNumBitfieldSubFieldContext {
            identifier: field.identifier.to_owned(),
            doc_lines: field.documentation.lines().map(|x| x.to_owned()).collect(),
            mask: format_u64_as_bin(mask),
            flag: format_u64_as_bin(flag),
        }]
    }
}

fn convert_value(
    value: &id_num::IntegerIdNumBitfieldSubFieldValue,
    identifier_prefix: String,
    bitfield_size: i8,
    index_offset: i8,
    mask: u64,
    flag: u64,
) -> Vec<IntegerIdNumBitfieldSubFieldContext> {
    let identifier = format!("{}{}", identifier_prefix, value.identifier);
    let mut all_fields: Vec<IntegerIdNumBitfieldSubFieldContext> =
        vec![IntegerIdNumBitfieldSubFieldContext {
            identifier: identifier.to_owned(),
            doc_lines: value.documentation.lines().map(|x| x.to_owned()).collect(),
            mask: format_u64_as_bin(mask),
            flag: format_u64_as_bin(flag),
        }];
    if !value.sub_fields.is_empty() {
        for v in &value.sub_fields {
            let fields = convert_field(
                &v,
                identifier.to_owned(),
                bitfield_size,
                index_offset,
                mask,
                flag,
            );
            all_fields.extend(fields);
        }
    }
    all_fields
}

//endregion