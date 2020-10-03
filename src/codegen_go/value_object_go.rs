//

use std::{error, fs, io::Write};

use crate::codegen_go::{BaseContext, GoCodeGenerator};

use azml::azml::{data_type, symbol, value_object::value_object};

impl GoCodeGenerator {
    pub fn generate_value_object_codes(
        &self,
        module_name: &String,
        symbol: &symbol::Symbol,
        vo: &value_object::ValueObject,
    ) -> Result<(), Box<dyn error::Error>> {
        let base_dir = &self.base_dir;
        let type_name = symbol.identifier.to_owned();

        let mut tpl_ctx = ValueObjectContext {
            base: self.render_base_context(),
            pkg_name: module_name.to_lowercase(),
            type_name: type_name.to_owned(),
            type_doc_lines: symbol.documentation.lines().map(|x| x.to_owned()).collect(),
            primitive_type_name: "".to_owned(),
            primitive_zero_value: "".to_owned(),
        };

        use data_type::DataType;
        let out_tpl_bytes: &[u8];
        if let Some(_vo_struct) = vo
            .definition
            .downcast_ref::<value_object::ValueObjectStruct>()
        {
            out_tpl_bytes = include_bytes!("templates/value_object_struct.gtmpl");
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
            tpl_ctx.primitive_type_name = prim_type;
            tpl_ctx.primitive_zero_value = prim_zero;
            out_tpl_bytes = include_bytes!("templates/value_object_alias.gtmpl");
        } else {
            out_tpl_bytes = "".as_bytes();
        }

        let out_code = gtmpl::template(
            String::from_utf8_lossy(out_tpl_bytes).as_ref(),
            tpl_ctx.to_owned(),
        )?;

        fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
        let mut service_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(format!("{}/{}/{}.go", base_dir, module_name, type_name,))?;
        service_file.write_all(out_code.as_bytes())?;

        Ok(())
    }
}

#[derive(Clone, Gtmpl)]
struct ValueObjectContext {
    base: BaseContext,
    pkg_name: String,
    type_name: String,
    type_doc_lines: Vec<String>,
    primitive_type_name: String,
    primitive_zero_value: String,
}
