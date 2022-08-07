//

use crate::convert_case::{Case, Casing};

pub fn render_template<T: Into<gtmpl_value::Value>>(
    template_str: &str,
    context: T,
) -> Result<String, String> {
    let mut tmpl = gtmpl::Template::default();
    tmpl.add_func("unexported_field", unexported_field);
    tmpl.add_func("unexported_global", unexported_global);
    tmpl.add_func("arg_name", arg_name);
    tmpl.add_func("sym_name", sym_name);
    tmpl.add_func("adjunct_host_id_db_col_name", adjunct_host_id_db_col_name);
    tmpl.add_func("attribute_db_col_name", attribute_db_col_name);
    tmpl.parse(template_str)?;
    tmpl.render(&gtmpl::Context::from(context)?)
}

fn unexported_field(args: &[gtmpl_value::Value]) -> Result<gtmpl_value::Value, String> {
    if let gtmpl_value::Value::String(ref o) = &args[0] {
        let s = o.split(".").last();
        let mut r = if let Some(c) = s {
            c.to_owned()
        } else {
            o.to_owned()
        };
        if let Some(r) = r.get_mut(0..1) {
            r.make_ascii_lowercase();
        }
        Ok(gtmpl_value::Value::String(r))
    } else {
        Err(format!("String required, got: {:?}", args))
    }
}

fn unexported_global(args: &[gtmpl_value::Value]) -> Result<gtmpl_value::Value, String> {
    if let gtmpl_value::Value::String(ref o) = &args[0] {
        let r: String = o.to_owned();
        Ok(gtmpl_value::Value::String(format!("_{}", r)))
    } else {
        Err(format!("String required, got: {:?}", args))
    }
}

fn arg_name(args: &[gtmpl_value::Value]) -> Result<gtmpl_value::Value, String> {
    if let gtmpl_value::Value::String(ref o) = &args[0] {
        let mut r: String = o.to_owned();
        if let Some(r) = r.get_mut(0..1) {
            r.make_ascii_lowercase();
        }
        Ok(gtmpl_value::Value::String(r))
    } else {
        Err(format!("String required, got: {:?}", args))
    }
}

fn sym_name(args: &[gtmpl_value::Value]) -> Result<gtmpl_value::Value, String> {
    if let gtmpl_value::Value::String(ref o) = &args[0] {
        let r: String = o.to_owned();
        let s = r.split(".").last();
        if let Some(x) = s {
            Ok(gtmpl_value::Value::String(x.to_owned()))
        } else {
            Err(format!("String required, got: {:?}", args))
        }
    } else {
        Err(format!("String required, got: {:?}", args))
    }
}

fn adjunct_host_id_db_col_name(args: &[gtmpl_value::Value]) -> Result<gtmpl_value::Value, String> {
    if let gtmpl_value::Value::String(ref o) = &args[0] {
        Ok(gtmpl_value::Value::String(o.to_case(Case::Snake) + "_id"))
    } else {
        Err(format!("String required, got: {:?}", args))
    }
}

fn attribute_db_col_name(args: &[gtmpl_value::Value]) -> Result<gtmpl_value::Value, String> {
    if let gtmpl_value::Value::String(ref o) = &args[0] {
        Ok(gtmpl_value::Value::String(o.to_case(Case::Snake)))
    } else {
        Err(format!("String required, got: {:?}", args))
    }
}
