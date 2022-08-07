//

use azml::azml::entity::id::id_num;

//region IntegerIdNumContext

#[derive(Clone, Gtmpl)]
pub struct IntegerIdNumContext {
    primitive_size: i8,
    primitive_size_bytes: i8,
    type_name: String,
    identifier_bits: i8,
    identifier_bits_mask: String,
    bitfield: IntegerIdNumBitfieldContext,
    pg_type: String,
}

impl From<&id_num::IntegerIdNum> for IntegerIdNumContext {
    fn from(x: &id_num::IntegerIdNum) -> Self {
        Self {
            primitive_size: x.primitive_size(),
            primitive_size_bytes: x.primitive_size() / 8,
            type_name: format!("int{}", x.primitive_size()),
            identifier_bits: x.identifier_bits,
            identifier_bits_mask: identifier_bit_mask_bin(x.primitive_size(), x.identifier_bits),
            // one for zero-based index, one for skipping the most-significant bit (the sign bit)
            bitfield: IntegerIdNumBitfieldContext::from(
                &x.bitfield,
                x.primitive_size(),
                x.primitive_size() - 2,
                0,
            ),
            pg_type: match x.primitive_size() {
                16 => "smallint".to_owned(),
                32 => "integer".to_owned(),
                64 => "bigint".to_owned(),
                _ => "".to_owned(),
            },
        }
    }
}

fn identifier_bit_mask_bin(total_size: i8, identifier_size: i8) -> String {
    let mut v: u64 = 0;
    for i in 0..total_size {
        if i < identifier_size {
            v |= 1 << i;
        }
    }
    format_u64_as_bin(v, total_size)
}

fn format_u64_as_bin(i: u64, width: i8) -> String {
    let i_str = format!("{:0width$b}", i, width = (width as usize));

    let a = i_str.chars().rev().enumerate();
    let mut s = String::new();
    for (idx, val) in a {
        if idx != 0 && idx % 8 == 0 {
            s.insert(0, '_');
        }
        s.insert(0, val);
    }

    format!("0b_{}", s)
}

//endregion

//region IntegerIdNumBitfieldContext

#[derive(Clone, Gtmpl)]
pub struct IntegerIdNumBitfieldContext {
    pub sub_fields: Vec<IntegerIdNumBitfieldSubFieldContext>,
    pub all_fields_mask: String,
}

impl IntegerIdNumBitfieldContext {
    fn from(
        x: &id_num::IntegerIdNumBitfield,
        total_size: i8,
        bitfield_size: i8,
        index_offset: i8,
    ) -> IntegerIdNumBitfieldContext {
        let mut all_fields: Vec<IntegerIdNumBitfieldSubFieldContext> = Vec::new();
        let mut idx: i8 = 0;
        let mut accum_mask: u64 = 0;
        for v in &x.sub_fields {
            let fields = convert_field(
                &v,
                "".to_owned(),
                total_size,
                bitfield_size,
                index_offset + idx,
                0,
                0,
            );

            for f in &fields {
                accum_mask |= f.mask_i;
            }

            all_fields.extend(fields);
            idx += 1;
        }
        IntegerIdNumBitfieldContext {
            sub_fields: all_fields,
            all_fields_mask: format_u64_as_bin(accum_mask, total_size),
        }
    }
}

//endregion

//region IntegerIdNumBitfieldSubFieldContext

#[derive(Clone, Gtmpl)]
pub struct IntegerIdNumBitfieldSubFieldContext {
    identifier: String,
    doc_lines: Vec<String>,
    mask_i: u64,
    mask: String,
    flag: String,
}

fn convert_field(
    field: &id_num::IntegerIdNumBitfieldSubField,
    identifier_prefix: String,
    total_size: i8,
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
                total_size,
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
            mask_i: mask,
            mask: format_u64_as_bin(mask, total_size),
            flag: format_u64_as_bin(flag, total_size),
        }]
    }
}

fn convert_value(
    value: &id_num::IntegerIdNumBitfieldSubFieldValue,
    identifier_prefix: String,
    total_size: i8,
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
            mask_i: mask,
            mask: format_u64_as_bin(mask, total_size),
            flag: format_u64_as_bin(flag, total_size),
        }];
    if !value.sub_fields.is_empty() {
        for v in &value.sub_fields {
            let fields = convert_field(
                &v,
                identifier.to_owned(),
                total_size,
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
