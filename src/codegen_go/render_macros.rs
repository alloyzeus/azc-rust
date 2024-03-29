//

#[macro_export]
macro_rules! render_file {
    ($target_dir: expr, $file_name_name: expr, $template_name: expr, $tpl_ctx: expr, $rendered_header: expr) => {
        fs::create_dir_all($target_dir)?;
        let out_tpl_bytes = include_bytes!($template_name);
        let out_code = render_template(
            String::from_utf8_lossy(out_tpl_bytes).as_ref(),
            $tpl_ctx.to_owned(),
        )?;
        let mut out_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!("{}/{}.go", $target_dir, $file_name_name))?;
        out_file.write_all($rendered_header.as_bytes())?;
        out_file.write_all(out_code.as_bytes())?;
        drop(out_file);
    };
}

#[macro_export]
macro_rules! render_file_append {
    ($out_file: expr, $template_name: expr, $tpl_ctx: expr) => {
        let out_tpl_bytes = include_bytes!($template_name);
        let out_code = render_template(
            String::from_utf8_lossy(out_tpl_bytes).as_ref(),
            $tpl_ctx.to_owned(),
        )?;
        $out_file.write_all("\n/**/ /**/ /**/ /**/\n\n".as_bytes())?;
        $out_file.write_all(out_code.as_bytes())?;
    };
}

#[macro_export]
macro_rules! render_file_region {
    ($out_file: expr, $region: expr, $template_name: expr, $tpl_ctx: expr) => {
        let out_tpl_bytes = include_bytes!($template_name);
        let out_code = render_template(
            String::from_utf8_lossy(out_tpl_bytes).as_ref(),
            $tpl_ctx.to_owned(),
        )?;
        $out_file.write_all("\n//region ".as_bytes())?;
        $out_file.write_all($region.as_bytes())?;
        $out_file.write_all("\n\n".as_bytes())?;
        $out_file.write_all(out_code.as_bytes())?;
        $out_file.write_all("\n//endregion\n".as_bytes())?;
    };
}
