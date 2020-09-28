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
    // The target directory path
    pub base_dir: String,
    // Go module identifier. This is the one defined in the go.mod file.
    pub module_identifier: String,
    pub azcore_import: String,
    pub azcore_pkg: String,
}

impl GoCodeGenerator {
    fn render_base_context(&self) -> BaseContext {
        BaseContext {
            mod_name: self.module_identifier.to_owned(),
            azcore_import: self.azcore_import.to_owned(),
            azcore_pkg: self.azcore_pkg.to_owned(),
        }
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
        let pkg_path = format!("{}/{}", self.module_identifier, module_name);
        let id_def = &ent.id.definition;
        if let Some(id_int) = id_def.downcast_ref::<entity_id_integer::EntityIdInteger>() {
            let id_size = Self::id_size_from_space(id_int.space);

            let id_type_name = format!("{}ID", identifier);
            let id_type_primitive = format!("int{}", id_size);
            let ref_key_type_name = format!("{}RefKey", identifier);
            let service_name = format!("{}Service", identifier);

            let tpl_ctx = EntityContext {
                base: self.render_base_context(),
                pkg_name: module_name.to_lowercase(),
                pkg_path: pkg_path.to_owned(),
                type_name: identifier.to_owned(),
                id_type_name: id_type_name.to_owned(),
                id_type_primitive: id_type_primitive.to_owned(),
                ref_key_type_name: ref_key_type_name.to_owned(),
                service_name: service_name.to_owned(),
            };

            // ID
            fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
            let out_name = format!("{}/{}/{}.go", base_dir, module_name, id_type_name,);
            let out_tpl_bytes = include_bytes!("entity_id.gtmpl");
            let out_code = gtmpl::template(
                String::from_utf8_lossy(out_tpl_bytes).as_ref(),
                tpl_ctx.to_owned(),
            )?;
            let mut out_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(out_name)?;
            out_file.write_all(out_code.as_bytes())?;
            drop(out_file);

            // RefKey
            fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
            let out_name = format!("{}/{}/{}.go", base_dir, module_name, ref_key_type_name,);
            let out_tpl_bytes = include_bytes!("entity_ref_key.gtmpl");
            let out_code = gtmpl::template(
                String::from_utf8_lossy(out_tpl_bytes).as_ref(),
                tpl_ctx.to_owned(),
            )?;
            let mut out_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(out_name)?;
            out_file.write_all(out_code.as_bytes())?;
            drop(out_file);

            // Service
            fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
            let out_name = format!("{}/{}/{}.go", base_dir, module_name, service_name,);
            let out_tpl_bytes = include_bytes!("entity_service.gtmpl");
            let out_code = gtmpl::template(
                String::from_utf8_lossy(out_tpl_bytes).as_ref(),
                tpl_ctx.to_owned(),
            )?;
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
            let out_tpl_bytes = include_bytes!("entity_service_base.gtmpl");
            let out_code = gtmpl::template(
                String::from_utf8_lossy(out_tpl_bytes).as_ref(),
                tpl_ctx.to_owned(),
            )?;
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
            let out_tpl_bytes = include_bytes!("entity_service_client_base.gtmpl");
            let out_code = gtmpl::template(
                String::from_utf8_lossy(out_tpl_bytes).as_ref(),
                tpl_ctx.to_owned(),
            )?;
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
            let out_tpl_bytes = include_bytes!("entity_service_server.gtmpl");
            let out_code = gtmpl::template(
                String::from_utf8_lossy(out_tpl_bytes).as_ref(),
                tpl_ctx.to_owned(),
            )?;
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
        let pkg_path = format!("{}/{}", self.module_identifier, module_name);
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

        let tpl_ctx = AdjunctEntityContext {
            base: self.render_base_context(),
            pkg_name: module_name.to_lowercase(),
            pkg_path: pkg_path.to_owned(),
            type_name: type_name.to_owned(),
            id_type_name: id_type_name.to_owned(),
            id_type_primitive: id_type_primitive.to_owned(),
            ref_key_type_name: ref_key_type_name.to_owned(),
            attributes_type_name: attrs_type_name.to_owned(),
            service_name: service_name.to_owned(),
        };

        // ID
        fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
        let out_name = format!("{}/{}/{}.go", base_dir, module_name, id_type_name,);
        let out_tpl_bytes = include_bytes!("adjunct_entity_id.gtmpl");
        let out_code = gtmpl::template(
            String::from_utf8_lossy(out_tpl_bytes).as_ref(),
            tpl_ctx.to_owned(),
        )?;
        let mut out_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(out_name)?;
        out_file.write_all(out_code.as_bytes())?;
        drop(out_file);

        // RefKey
        fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
        let out_name = format!("{}/{}/{}.go", base_dir, module_name, ref_key_type_name,);
        let out_tpl_bytes = include_bytes!("adjunct_entity_ref_key.gtmpl");
        let out_code = gtmpl::template(
            String::from_utf8_lossy(out_tpl_bytes).as_ref(),
            tpl_ctx.to_owned(),
        )?;
        let mut out_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(out_name)?;
        out_file.write_all(out_code.as_bytes())?;
        drop(out_file);

        // Attributes
        fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
        let out_name = format!("{}/{}/{}.go", base_dir, module_name, attrs_type_name,);
        let out_tpl_bytes = include_bytes!("adjunct_entity_attributes.gtmpl");
        let out_code = gtmpl::template(
            String::from_utf8_lossy(out_tpl_bytes).as_ref(),
            tpl_ctx.to_owned(),
        )?;
        let mut out_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(out_name)?;
        out_file.write_all(out_code.as_bytes())?;
        drop(out_file);

        // Service
        fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
        let out_name = format!("{}/{}/{}.go", base_dir, module_name, service_name,);
        let out_tpl_bytes = include_bytes!("adjunct_entity_service.gtmpl");
        let out_code = gtmpl::template(
            String::from_utf8_lossy(out_tpl_bytes).as_ref(),
            tpl_ctx.to_owned(),
        )?;
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

        let mut tpl_ctx = ValueObjectContext {
            base: self.render_base_context(),
            pkg_name: module_name.to_lowercase(),
            type_name: identifier.to_owned(),
            primitive_type_name: "".to_owned(),
        };

        use data_type::DataType;
        let out_tpl_bytes = match vo.data_type {
            DataType::Struct => include_bytes!("value_object_struct.gtmpl"),
            _ => {
                let prim_type = match vo.data_type {
                    DataType::Int8 => "int8".to_owned(),
                    DataType::Int16 => "int16".to_owned(),
                    DataType::Int32 => "int32".to_owned(),
                    DataType::Int64 => "int64".to_owned(),
                    DataType::String => "string".to_owned(),
                    DataType::Struct => "struct".to_owned(),
                };
                tpl_ctx.primitive_type_name = prim_type;
                include_bytes!("value_object_primitive.gtmpl")
            }
        };

        let out_code = gtmpl::template(
            String::from_utf8_lossy(out_tpl_bytes).as_ref(),
            tpl_ctx.to_owned(),
        )?;

        fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
        let mut service_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(format!("{}/{}/{}.go", base_dir, module_name, identifier,))?;
        service_file.write_all(out_code.as_bytes())?;

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
        let tpl_ctx = LibraryContext {
            base: self.render_base_context(),
            pkg_name: module_name.to_owned(),
        };
        fs::create_dir_all(format!("{}/{}", base_dir, module_name,))?;
        let out_name = format!("{}/{}/AZEntityService.go", base_dir, module_name,);
        let out_tpl_bytes = include_bytes!("az_entity_service.gtmpl");
        let out_code = gtmpl::template(
            String::from_utf8_lossy(out_tpl_bytes).as_ref(),
            tpl_ctx.to_owned(),
        )?;
        let mut out_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(out_name)?;
        out_file.write_all(out_code.as_bytes())?;
        drop(out_file);

        for symbol in &module_def.symbols {
            let params = &symbol.definition;
            if let Some(ent) = params.downcast_ref::<entity::Entity>() {
                self.generate_entity_codes(module_name, ent, &symbol.identifier)?;
                continue;
            }
            if let Some(adj) = params.downcast_ref::<adjunct::Adjunct>() {
                if let Some(adj_ent) = adj
                    .definition
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
                    .definition
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

#[derive(Clone, Gtmpl)]
struct BaseContext {
    mod_name: String,
    azcore_import: String,
    azcore_pkg: String,
}

#[derive(Clone, Gtmpl)]
struct LibraryContext {
    base: BaseContext,
    pkg_name: String,
}

#[derive(Clone, Gtmpl)]
struct EntityContext {
    base: BaseContext,
    pkg_name: String,
    pkg_path: String,
    type_name: String,
    id_type_name: String,
    id_type_primitive: String,
    ref_key_type_name: String,
    service_name: String,
}

#[derive(Clone, Gtmpl)]
struct AdjunctEntityContext {
    base: BaseContext,
    pkg_name: String,
    pkg_path: String,
    type_name: String,
    id_type_name: String,
    id_type_primitive: String,
    ref_key_type_name: String,
    attributes_type_name: String,
    service_name: String,
}

#[derive(Clone, Gtmpl)]
struct ValueObjectContext {
    base: BaseContext,
    pkg_name: String,
    type_name: String,
    primitive_type_name: String,
}
