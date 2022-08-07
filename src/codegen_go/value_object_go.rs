//

use std::{error, fs, io::Write};

use crate::codegen_go::{template::render_template, BaseContext, GoCodeGenerator};

use azml::azml::{data_type, symbol, value_object::value_object};

impl GoCodeGenerator {
    pub fn generate_value_object_codes(
        &self,
        module_name: &String,
        symbol: &symbol::Symbol,
        vo: &value_object::ValueObject,
    ) -> Result<(), Box<dyn error::Error>> {
        let type_name = symbol.identifier.to_owned();

        use data_type::DataType;
        let out_code: String;

        if let Some(vo_struct) = vo
            .definition
            .downcast_ref::<value_object::ValueObjectStruct>()
        {
            let tpl_ctx = ValueObjectStructContext {
                base: self.render_base_context(),
                pkg_name: module_name.to_lowercase(),
                type_name: type_name.to_owned(),
                type_doc_lines: symbol.documentation.lines().map(|x| x.to_owned()).collect(),
                fields: vo_struct
                    .fields
                    .clone()
                    .into_iter()
                    .map(|x| ValueObjectStructFieldContext {
                        identifier: x.identifier.to_owned(),
                        type_name: x.data_type.into(),
                    })
                    .collect::<Vec<ValueObjectStructFieldContext>>(),
            };
            let out_tpl_bytes = include_bytes!("templates/value_object_struct.gtmpl");

            out_code = render_template(
                String::from_utf8_lossy(out_tpl_bytes).as_ref(),
                tpl_ctx.to_owned(),
            )?;
        } else if let Some(vo_alias) = vo
            .definition
            .downcast_ref::<value_object::ValueObjectAlias>()
        {
            let (prim_type, prim_zero) = match vo_alias.data_type {
                DataType::Int8 => ("int8".to_owned(), "0".to_owned()),
                DataType::Int16 => ("int16".to_owned(), "0".to_owned()),
                DataType::Int32 => ("int32".to_owned(), "0".to_owned()),
                DataType::Int64 => ("int64".to_owned(), "0".to_owned()),
                DataType::String => ("string".to_owned(), r#""""#.to_owned()),
                DataType::Bytes => ("[]byte".to_owned(), r#""""#.to_owned()),
            };
            let tpl_ctx = ValueObjectAliasContext {
                base: self.render_base_context(),
                pkg_name: module_name.to_lowercase(),
                type_name: type_name.to_owned(),
                type_doc_lines: symbol.documentation.lines().map(|x| x.to_owned()).collect(),
                primitive_type_name: prim_type,
                primitive_zero_value: prim_zero,
            };
            let out_tpl_bytes = include_bytes!("templates/value_object_alias.gtmpl");

            out_code = render_template(
                String::from_utf8_lossy(out_tpl_bytes).as_ref(),
                tpl_ctx.to_owned(),
            )?;
        } else {
            out_code = "".to_string();
        }

        fs::create_dir_all(self.package_dir_base_name.to_owned())?;
        let mut service_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(format!("{}/{}.go", self.package_dir_base_name, type_name,))?;
        service_file.write_all(out_code.as_bytes())?;

        Ok(())
    }
}

#[derive(Clone, Gtmpl)]
struct ValueObjectAliasContext {
    base: BaseContext,
    pkg_name: String,
    type_name: String,
    type_doc_lines: Vec<String>,
    primitive_type_name: String,
    primitive_zero_value: String,
}

#[derive(Clone, Gtmpl)]
struct ValueObjectStructContext {
    base: BaseContext,
    pkg_name: String,
    type_name: String,
    type_doc_lines: Vec<String>,
    fields: Vec<ValueObjectStructFieldContext>,
}

#[derive(Clone, Gtmpl)]
struct ValueObjectStructFieldContext {
    identifier: String,
    type_name: String,
}
