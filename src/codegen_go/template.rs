//

pub fn render_template<T: Into<gtmpl_value::Value>>(
    template_str: &str,
    context: T,
) -> Result<String, String> {
    let mut tmpl = gtmpl::Template::default();
    tmpl.add_func("unexported_field", unexported_field);
    tmpl.add_func("arg_name", arg_name);
    tmpl.add_func("sym_name", sym_name);
    tmpl.parse(template_str)?;
    tmpl.render(&gtmpl::Context::from(context)?)
}

fn unexported_field(args: &[gtmpl_value::Value]) -> Result<gtmpl_value::Value, String> {
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
