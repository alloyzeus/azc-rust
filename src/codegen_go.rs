//

use std::{error, fs, io::Write};

use crate::codegen;

use azml::azml::{
    adjunct::{adjunct, adjunct_entity, adjunct_value_object},
    data_type,
    entity::{entity, entity_id_integer},
    module,
    value_object::value_object,
};

pub struct GoCodeGenerator {
    pub base_dir: String,
    pub go_module_name: String,
    pub azcore_import: String,
    pub azcore_name: String,
}

impl GoCodeGenerator {
    fn new_template_context(&self) -> mhtemplate::Context {
        let mut mht_ctx = mhtemplate::Context::new();
        mht_ctx["MOD_NAME"] = self.go_module_name.to_owned();
        mht_ctx["AZCORE_IMPORT"] = self.azcore_import.to_owned();
        mht_ctx["AZCORE_PKG"] = self.azcore_name.to_owned();
        mht_ctx
    }

    fn id_size_from_space(id_space: i8) -> i8 {
        match id_space {
            d if d < 16 => 16,
            d if d < 32 => 32,
            d if d < 64 => 64,
            _ => -1, //TODO: error. we won't need this here. generators receive clean data.
        }
    }

    fn generate_entity_codes(
        &self,
        module_name: &String,
        ent: &entity::Entity,
        identifier: &String,
    ) -> Result<(), Box<dyn error::Error>> {
        let base_dir = &self.base_dir;
        let pkg_path = format!("{}/{}", self.go_module_name, module_name);
        let id_def = &ent.id.parameters;
        if let Some(id_int) = id_def.downcast_ref::<entity_id_integer::EntityIdInteger>() {
            let id_size = Self::id_size_from_space(id_int.space);

            let id_type_name = format!("{}ID", identifier);
            let id_type_primitive = format!("int{}", id_size);
            let ref_key_type_name = format!("{}RefKey", identifier);
            let service_name = format!("{}Service", identifier);

            let mut mht_ctx = self.new_template_context();
            mht_ctx["PKG_NAME"] = module_name.to_lowercase();
            mht_ctx["PKG_PATH"] = pkg_path.to_owned();
            mht_ctx["TYPE_NAME"] = identifier.to_owned();
            mht_ctx["ID_TYPE_NAME"] = id_type_name.to_owned();
            mht_ctx["ID_TYPE_PRIMITIVE"] = id_type_primitive;
            mht_ctx["REF_KEY_TYPE_NAME"] = ref_key_type_name.to_owned();
            mht_ctx["SERVICE_NAME"] = service_name.to_owned();

            // ID
            fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
            let out_name = format!("{}/{}/{}.go", base_dir, module_name, id_type_name,);
            let out_tpl_bytes = include_bytes!("entity_id.got");
            let out_tpl =
                mhtemplate::TemplateFactory::new(String::from_utf8_lossy(out_tpl_bytes).as_ref())
                    .parse()?;
            let out_code = out_tpl.evaluate(&mut mht_ctx)?;
            let mut out_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(out_name)?;
            out_file.write_all(out_code.as_bytes())?;
            drop(out_file);

            // RefKey
            fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
            let out_name = format!("{}/{}/{}.go", base_dir, module_name, ref_key_type_name,);
            let out_tpl_bytes = include_bytes!("entity_ref_key.got");
            let out_tpl =
                mhtemplate::TemplateFactory::new(String::from_utf8_lossy(out_tpl_bytes).as_ref())
                    .parse()?;
            let out_code = out_tpl.evaluate(&mut mht_ctx)?;
            let mut out_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(out_name)?;
            out_file.write_all(out_code.as_bytes())?;
            drop(out_file);

            // Service
            fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
            let out_name = format!("{}/{}/{}.go", base_dir, module_name, service_name,);
            let out_tpl_bytes = include_bytes!("entity_service.got");
            let out_tpl =
                mhtemplate::TemplateFactory::new(String::from_utf8_lossy(out_tpl_bytes).as_ref())
                    .parse()?;
            let out_code = out_tpl.evaluate(&mut mht_ctx)?;
            let mut out_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(out_name)?;
            out_file.write_all(out_code.as_bytes())?;
            drop(out_file);

            if !ent.attributes.is_empty() {
                println!("TODO: attributes for entity {}", identifier);
            }

            // Service
            fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
            let out_name = format!("{}/{}/{}Base.go", base_dir, module_name, service_name,);
            let out_tpl_bytes = include_bytes!("entity_service_base.got");
            let out_tpl =
                mhtemplate::TemplateFactory::new(String::from_utf8_lossy(out_tpl_bytes).as_ref())
                    .parse()?;
            let out_code = out_tpl.evaluate(&mut mht_ctx)?;
            let mut out_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(out_name)?;
            out_file.write_all(out_code.as_bytes())?;
            drop(out_file);

            // ServiceClient
            fs::create_dir_all(format!("{}/{}/client", base_dir, module_name,))?;
            let out_name = format!(
                "{}/{}/client/{}Base.go",
                base_dir, module_name, service_name,
            );
            let out_tpl_bytes = include_bytes!("entity_service_client_base.got");
            let out_tpl =
                mhtemplate::TemplateFactory::new(String::from_utf8_lossy(out_tpl_bytes).as_ref())
                    .parse()?;
            let out_code = out_tpl.evaluate(&mut mht_ctx)?;
            let mut out_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(out_name)?;
            out_file.write_all(out_code.as_bytes())?;
            drop(out_file);

            // ServiceClient
            fs::create_dir_all(format!("{}/{}server", base_dir, module_name,))?;
            let out_name = format!(
                "{}/{}server/{}Server.go",
                base_dir, module_name, service_name,
            );
            let out_tpl_bytes = include_bytes!("entity_service_server.got");
            let out_tpl =
                mhtemplate::TemplateFactory::new(String::from_utf8_lossy(out_tpl_bytes).as_ref())
                    .parse()?;
            let out_code = out_tpl.evaluate(&mut mht_ctx)?;
            let mut out_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(out_name)?;
            out_file.write_all(out_code.as_bytes())?;
            drop(out_file);
        }
        Ok(())
    }

    fn generate_adjunct_entity_codes(
        &self,
        module_name: &String,
        _adj_ent: &adjunct_entity::AdjunctEntity,
        identifier: &String,
        hosts: &Vec<adjunct::AdjunctHost>,
    ) -> Result<(), Box<dyn error::Error>> {
        let base_dir = &self.base_dir;
        let base_type_name = hosts
            .into_iter()
            .map(|x| x.name.to_owned())
            .collect::<Vec<String>>()
            .join("");

        let type_name = format!("{}{}", base_type_name, identifier);
        let id_type_name = format!("{}ID", type_name);
        let id_type_primitive = format!("int{}", 64); //TODO: de-hardcode
        let ref_key_type_name = format!("{}RefKey", type_name);
        let attrs_type_name = format!("{}Attributes", type_name);
        let service_name = format!("{}Service", type_name);

        let mut mht_ctx = self.new_template_context();
        mht_ctx["PKG_NAME"] = module_name.to_lowercase();
        mht_ctx["TYPE_NAME"] = type_name.to_owned();
        mht_ctx["ID_TYPE_NAME"] = id_type_name.to_owned();
        mht_ctx["ID_TYPE_PRIMITIVE"] = id_type_primitive;
        mht_ctx["REF_KEY_TYPE_NAME"] = ref_key_type_name.to_owned();
        mht_ctx["ATTRIBUTES_TYPE_NAME"] = attrs_type_name.to_owned();
        mht_ctx["SERVICE_NAME"] = service_name.to_owned();

        // ID
        fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
        let out_name = format!("{}/{}/{}.go", base_dir, module_name, id_type_name,);
        let out_tpl_bytes = include_bytes!("adjunct_entity_id.got");
        let out_tpl =
            mhtemplate::TemplateFactory::new(String::from_utf8_lossy(out_tpl_bytes).as_ref())
                .parse()?;
        let out_code = out_tpl.evaluate(&mut mht_ctx)?;
        let mut out_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(out_name)?;
        out_file.write_all(out_code.as_bytes())?;
        drop(out_file);

        // RefKey
        fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
        let out_name = format!("{}/{}/{}.go", base_dir, module_name, ref_key_type_name,);
        let out_tpl_bytes = include_bytes!("adjunct_entity_ref_key.got");
        let out_tpl =
            mhtemplate::TemplateFactory::new(String::from_utf8_lossy(out_tpl_bytes).as_ref())
                .parse()?;
        let out_code = out_tpl.evaluate(&mut mht_ctx)?;
        let mut out_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(out_name)?;
        out_file.write_all(out_code.as_bytes())?;
        drop(out_file);

        // Attributes
        fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
        let out_name = format!("{}/{}/{}.go", base_dir, module_name, attrs_type_name,);
        let out_tpl_bytes = include_bytes!("adjunct_entity_attributes.got");
        let out_tpl =
            mhtemplate::TemplateFactory::new(String::from_utf8_lossy(out_tpl_bytes).as_ref())
                .parse()?;
        let out_code = out_tpl.evaluate(&mut mht_ctx)?;
        let mut out_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(out_name)?;
        out_file.write_all(out_code.as_bytes())?;
        drop(out_file);

        // Service
        fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
        let out_name = format!("{}/{}/{}.go", base_dir, module_name, service_name,);
        let out_tpl_bytes = include_bytes!("adjunct_entity_service.got");
        let out_tpl =
            mhtemplate::TemplateFactory::new(String::from_utf8_lossy(out_tpl_bytes).as_ref())
                .parse()?;
        let out_code = out_tpl.evaluate(&mut mht_ctx)?;
        let mut out_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(out_name)?;
        out_file.write_all(out_code.as_bytes())?;
        drop(out_file);

        Ok(())
    }

    fn generate_value_object_codes(
        &self,
        module_name: &String,
        vo: &value_object::ValueObject,
        identifier: &String,
    ) -> Result<(), Box<dyn error::Error>> {
        let base_dir = &self.base_dir;
        let mut mht_ctx = self.new_template_context();
        mht_ctx["PKG_NAME"] = module_name.to_lowercase();
        mht_ctx["TYPE_NAME"] = identifier.to_owned();

        let tpl: Box<dyn mhtemplate::Template>;

        use data_type::DataType;
        match vo.data_type {
            DataType::Struct => {
                let out_tpl_bytes = include_bytes!("value_object_struct.got");
                tpl = mhtemplate::TemplateFactory::new(
                    String::from_utf8_lossy(out_tpl_bytes).as_ref(),
                )
                .parse()?;
            }
            _ => {
                let out_tpl_bytes = include_bytes!("value_object_primitive.got");
                tpl = mhtemplate::TemplateFactory::new(
                    String::from_utf8_lossy(out_tpl_bytes).as_ref(),
                )
                .parse()?;

                let prim_type = match vo.data_type {
                    DataType::Int8 => "int8".to_owned(),
                    DataType::Int16 => "int16".to_owned(),
                    DataType::Int32 => "int32".to_owned(),
                    DataType::Int64 => "int64".to_owned(),
                    DataType::String => "string".to_owned(),
                    DataType::Struct => "struct".to_owned(),
                };
                mht_ctx["PRIMITIVE_TYPE_NAME"] = prim_type;
            }
        }

        let service_code = tpl.evaluate(&mut mht_ctx)?;
        fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
        let mut service_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(format!("{}/{}/{}.go", base_dir, module_name, identifier,))?;
        service_file.write_all(service_code.as_bytes())?;

        Ok(())
    }
}

impl codegen::CodeGenerator for GoCodeGenerator {
    fn generate_module_codes(
        &self,
        module_name: &String,
        module_def: &module::ModuleDefinition,
    ) -> Result<(), Box<dyn error::Error>> {
        let base_dir = self.base_dir.to_owned();
        let mut mht_ctx = self.new_template_context();
        mht_ctx["PKG_NAME"] = module_name.to_owned();
        fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
        let out_name = format!("{}/{}/AZEntityService.go", base_dir, module_name,);
        let out_tpl_bytes = include_bytes!("az_entity_service.got");
        let out_tpl =
            mhtemplate::TemplateFactory::new(String::from_utf8_lossy(out_tpl_bytes).as_ref())
                .parse()?;
        let out_code = out_tpl.evaluate(&mut mht_ctx)?;
        let mut out_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(out_name)?;
        out_file.write_all(out_code.as_bytes())?;
        drop(out_file);

        for symbol in &module_def.symbols {
            let params = &symbol.parameters;
            if let Some(ent) = params.downcast_ref::<entity::Entity>() {
                self.generate_entity_codes(module_name, ent, &symbol.identifier)?;
                continue;
            }
            if let Some(adj) = params.downcast_ref::<adjunct::Adjunct>() {
                if let Some(adj_ent) = adj
                    .parameters
                    .downcast_ref::<adjunct_entity::AdjunctEntity>()
                {
                    self.generate_adjunct_entity_codes(
                        module_name,
                        adj_ent,
                        &symbol.identifier,
                        &adj.hosts,
                    )?;
                    continue;
                }
                if let Some(adj_vo) = adj
                    .parameters
                    .downcast_ref::<adjunct_value_object::AdjunctValueObject>()
                {
                    println!("TODO: Value-object entity adjunct {:?}", adj_vo);
                    continue;
                }
                continue;
            }
            if let Some(vo) = params.downcast_ref::<value_object::ValueObject>() {
                self.generate_value_object_codes(module_name, vo, &symbol.identifier)?;
                continue;
            }
        }
        Ok(())
    }
}
