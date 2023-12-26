//

use crate::convert_case::{Case, Casing};

pub fn render_template<T: Into<gtmpl::Value>>(
    template_str: &str,
    context: T,
) -> Result<String, String> {
    let mut tmpl = gtmpl::Template::default();
    tmpl.add_func("unexported_field", unexported_field);
    tmpl.add_func("unexported_global", unexported_global);
    tmpl.add_func("arg_name", arg_name);
    tmpl.add_func("sym_name", sym_name);
    tmpl.add_func("comparable", comparable);
    tmpl.add_func("is_slice", is_slice);
    tmpl.add_func("adjunct_host_id_db_col_name", adjunct_host_id_db_col_name);
    tmpl.add_func("attribute_db_col_name", attribute_db_col_name);
    tmpl.parse(template_str).map_err(|e| e.to_string())?;
    tmpl.render(&gtmpl::Context::from(context))
        .map_err(|e| e.to_string())
}

pub fn go_unexport(o: &String) -> String {
    let s = o.split(".").last();
    let mut r = if let Some(c) = s {
        c.to_owned()
    } else {
        o.to_owned()
    };
    if let Some(r) = r.get_mut(0..1) {
        r.make_ascii_lowercase();
    }
    r
}

fn unexported_field(args: &[gtmpl::Value]) -> Result<gtmpl::Value, gtmpl::FuncError> {
    if let gtmpl::Value::String(ref o) = &args[0] {
        let r = go_unexport(o);
        Ok(gtmpl::Value::String(r))
    } else {
        Err(gtmpl::FuncError::UnableToConvertFromValue)
    }
}

fn unexported_global(args: &[gtmpl::Value]) -> Result<gtmpl::Value, gtmpl::FuncError> {
    if let gtmpl::Value::String(ref o) = &args[0] {
        let r: String = o.to_owned();
        Ok(gtmpl::Value::String(format!("_{}", r)))
    } else {
        Err(gtmpl::FuncError::UnableToConvertFromValue)
    }
}

fn arg_name(args: &[gtmpl::Value]) -> Result<gtmpl::Value, gtmpl::FuncError> {
    if let gtmpl::Value::String(ref o) = &args[0] {
        let mut r: String = o.to_owned();
        if let Some(r) = r.get_mut(0..1) {
            r.make_ascii_lowercase();
        }
        Ok(gtmpl::Value::String(r))
    } else {
        Err(gtmpl::FuncError::UnableToConvertFromValue)
    }
}

fn sym_name(args: &[gtmpl::Value]) -> Result<gtmpl::Value, gtmpl::FuncError> {
    if let gtmpl::Value::String(ref o) = &args[0] {
        let r: String = o.to_owned();
        let s = r.split(".").last();
        if let Some(x) = s {
            Ok(gtmpl::Value::String(x.to_owned()))
        } else {
            Err(gtmpl::FuncError::UnableToConvertFromValue)
        }
    } else {
        Err(gtmpl::FuncError::UnableToConvertFromValue)
    }
}

fn comparable(args: &[gtmpl::Value]) -> Result<gtmpl::Value, gtmpl::FuncError> {
    if let gtmpl::Value::String(ref o) = &args[0] {
        let r: String = o.to_owned();
        match r.as_str() {
            "string" => Ok(gtmpl::Value::Bool(true)),
            "int64" => Ok(gtmpl::Value::Bool(true)),
            "int32" => Ok(gtmpl::Value::Bool(true)),
            _ => Ok(gtmpl::Value::Bool(false)),
        }
    } else {
        Err(gtmpl::FuncError::UnableToConvertFromValue)
    }
}

fn is_slice(args: &[gtmpl::Value]) -> Result<gtmpl::Value, gtmpl::FuncError> {
    if let gtmpl::Value::String(ref o) = &args[0] {
        let r: String = o.to_owned();
        Ok(gtmpl::Value::Bool(r.starts_with("[]")))
    } else {
        Err(gtmpl::FuncError::UnableToConvertFromValue)
    }
}

fn adjunct_host_id_db_col_name(args: &[gtmpl::Value]) -> Result<gtmpl::Value, gtmpl::FuncError> {
    if let gtmpl::Value::String(ref o) = &args[0] {
        Ok(gtmpl::Value::String(o.to_case(Case::Snake) + "_id"))
    } else {
        Err(gtmpl::FuncError::UnableToConvertFromValue)
    }
}

fn attribute_db_col_name(args: &[gtmpl::Value]) -> Result<gtmpl::Value, gtmpl::FuncError> {
    if let gtmpl::Value::String(ref o) = &args[0] {
        Ok(gtmpl::Value::String(o.to_case(Case::Snake)))
    } else {
        Err(gtmpl::FuncError::UnableToConvertFromValue)
    }
}
