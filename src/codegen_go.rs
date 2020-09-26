//

use std::{error, fs, io::Write};

use crate::codegen;

use azml::azml::{
    adjunct::{adjunct, adjunct_entity},
    data_type,
    entity::{entity, entity_id_integer},
    module,
    value_object::value_object,
};

pub struct GoCodeGenerator {
    pub base_dir: String,
}

impl GoCodeGenerator {}

impl GoCodeGenerator {
    fn generate_entity_codes(
        &self,
        module_name: &String,
        ent: &entity::Entity,
        identifier: &String,
    ) -> Result<(), Box<dyn error::Error>> {
        let base_dir = &self.base_dir;
        let id_def = &ent.id.parameters;
        if let Some(id_int) = id_def.downcast_ref::<entity_id_integer::EntityIdInteger>() {
            let id_size = match id_int.space {
                d if d < 16 => 16,
                d if d < 32 => 32,
                d if d < 64 => 64,
                _ => -1, //TODO: error. we won't need this here. generators receive clean data.
            };

            let id_type_name = format!("{}ID", identifier);
            let id_type_primitive = format!("int{}", id_size);
            let service_name = format!("{}Service", identifier);

            let mut mht_ctx = mhtemplate::Context::new();
            mht_ctx["PACKAGE_NAME"] = module_name.to_lowercase();
            mht_ctx["TYPE_NAME"] = identifier.to_owned();
            mht_ctx["ID_TYPE_NAME"] = id_type_name.to_owned();
            mht_ctx["ID_TYPE_PRIMITIVE"] = id_type_primitive;
            mht_ctx["SERVICE_NAME"] = service_name.to_owned();

            // filename: ./<module>/<identifier>_id.go
            let id_tpl = mhtemplate::TemplateFactory::new(
                    "package {{$PACKAGE_NAME}}\n\
                    \n\
                    // {{$ID_TYPE_NAME}} is used to identify an instance of {{$TYPE_NAME}}.\n\
                    type {{$ID_TYPE_NAME}} {{$ID_TYPE_PRIMITIVE}}\n\
                    \n\
                    // {{$ID_TYPE_NAME}}Zero is the zero value for entity {{$TYPE_NAME}}.\n\
                    const {{$ID_TYPE_NAME}}Zero = {{$ID_TYPE_NAME}}(0)\n\
                    \n\
                    func {{$ID_TYPE_NAME}}FromPrimitiveValue(v {{$ID_TYPE_PRIMITIVE}}) {{$ID_TYPE_NAME}} { return {{$ID_TYPE_NAME}}(v) }\n\
                    func (id {{$ID_TYPE_NAME}}) PrimitiveValue() {{$ID_TYPE_PRIMITIVE}} { return {{$ID_TYPE_PRIMITIVE}}(id) }\n\
                    \n\
                    \n").parse().unwrap();

            let id_code = id_tpl.evaluate(&mut mht_ctx).unwrap();
            fs::create_dir_all(format!("{}/{}", base_dir, module_name,)).unwrap();
            let mut id_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(format!("{}/{}/{}.go", base_dir, module_name, id_type_name,))
                .unwrap();
            id_file.write_all(id_code.as_bytes()).unwrap();

            // filename: ./<module>/<identifier>_service.go
            let service_tpl = mhtemplate::TemplateFactory::new(
                    "package {{$PACKAGE_NAME}}\n\
                    \n\
                    // {{$SERVICE_NAME}} provides a contract for methods related to entity {{$TYPE_NAME}}.\n\
                    type {{$SERVICE_NAME}} interface {\n\
                    \tListen{{$TYPE_NAME}}Events({{$TYPE_NAME}}EventsListenInput) {{$TYPE_NAME}}EventsListenInput\n\
                    \n\
                    \tCreate{{$TYPE_NAME}}({{$TYPE_NAME}}CreateInput) {{$TYPE_NAME}}CreateOutput\n\
                    \n\
                    \t// TODO: other stuff from the mixins\n\
                    }\n\
                    \n\
                    // {{$TYPE_NAME}}CreateInput is for use in Create{{$TYPE_NAME}} method of {{$SERVICE_NAME}}.\n\
                    type {{$TYPE_NAME}}CreateInput struct {\n\
                    \tContext AZEntityCreationInputContext\n\
                    \tParameters {{$TYPE_NAME}}CreateInputParameters\n\
                    }\n\
                    \n\
                    type {{$TYPE_NAME}}CreateInputParameters struct {\n\
                    \t// TODO\n\
                    }\n\
                    \n\
                    type {{$TYPE_NAME}}CreateOutput struct {\n\
                    \tContext AZEntityCreationOutputContext\n\
                    \tParameters {{$TYPE_NAME}}CreateOutputParameters\n\
                    }\n\
                    \n\
                    type {{$TYPE_NAME}}CreateOutputParameters struct {\n\
                    \tID {{$ID_TYPE_NAME}}\n\
                    \tCreationInfo *AZEntityCreationInfo\n\
                    \n\
                    \t// TODO\n\
                    }\n\
                    \n\
                    type {{$TYPE_NAME}}CreateEvent struct {\n\
                    \tAZEntityCreationEventBase\n\
                    \t// TODO\n\
                    }\n\
                    \n",
                ).parse().unwrap();

            let service_code = service_tpl.evaluate(&mut mht_ctx).unwrap();
            fs::create_dir_all(format!("{}/{}", base_dir, module_name,)).unwrap();
            let mut id_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(format!("{}/{}/{}.go", base_dir, module_name, service_name,))
                .unwrap();
            id_file.write_all(service_code.as_bytes()).unwrap();

            // filename: ./<module>/<identifier>_service_base.go
            let service_base_tpl = mhtemplate::TemplateFactory::new(
                    "package {{$PACKAGE_NAME}}\n\
                    \n\
                    // {{$SERVICE_NAME}}Base is the base implementation for {{$SERVICE_NAME}} shared by client and server.\n\
                    type {{$SERVICE_NAME}}Base struct {\n\
                    \t//TODO: implement this.\n\
                    }\n\
                    \n",
                )
                .parse()
                .unwrap();

            let service_base_code = service_base_tpl.evaluate(&mut mht_ctx).unwrap();
            fs::create_dir_all(format!("{}/{}", base_dir, module_name,)).unwrap();
            let mut id_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(format!(
                    "{}/{}/{}Base.go",
                    base_dir, module_name, service_name,
                ))
                .unwrap();
            id_file.write_all(service_base_code.as_bytes()).unwrap();

            // filename: ./<module>/<module>client/<identifier>_service_client.go
            let client_tpl = mhtemplate::TemplateFactory::new(
                    "package client\n\
                    \n\
                    // {{$SERVICE_NAME}}ClientBase is the base client implementation for {{$SERVICE_NAME}}\n\
                    type {{$SERVICE_NAME}}ClientBase struct {\n\
                    \t// Embed shared service implementation.\n\
                    \t{{$PACKAGE_NAME}}.{{$SERVICE_NAME}}Base\n\
                    \n\
                    \t//TODO: implement this.\n\
                    }\n\
                    \n\
                    var _ {{$PACKAGE_NAME}}.{{$SERVICE_NAME}} = &{{$SERVICE_NAME}}ClientBase{}\n\
                    \n",
                )
                .parse()
                .unwrap();

            let client_code = client_tpl.evaluate(&mut mht_ctx).unwrap();
            fs::create_dir_all(format!("{}/{}/client", base_dir, module_name,)).unwrap();
            let mut id_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(format!(
                    "{}/{}/client/{}Base.go",
                    base_dir, module_name, service_name,
                ))
                .unwrap();
            id_file.write_all(client_code.as_bytes()).unwrap();

            // filename: ./<module>/<identifier>server/<identifier>_service_server.go
            let server_tpl = mhtemplate::TemplateFactory::new(
                "package {{$PACKAGE_NAME}}server\n\
                    \n\
                    // {{$SERVICE_NAME}}Server is the server implementation for {{$SERVICE_NAME}}\n\
                    type {{$SERVICE_NAME}}Server struct {\n\
                    \t// Embed shared service implementation.\n\
                    \t{{$PACKAGE_NAME}}.{{$SERVICE_NAME}}Base\n\
                    \n\
                    \t//TODO: implement this.\n\
                    }\n\
                    \n\
                    var _ {{$PACKAGE_NAME}}.{{$SERVICE_NAME}} = &{{$SERVICE_NAME}}Server{}\n\
                    \n",
            )
            .parse()
            .unwrap();

            let server_code = server_tpl.evaluate(&mut mht_ctx).unwrap();
            fs::create_dir_all(format!("{}/{}server", base_dir, module_name,)).unwrap();
            let mut id_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(format!(
                    "{}/{}server/{}Server.go",
                    base_dir, module_name, service_name,
                ))
                .unwrap();
            id_file.write_all(server_code.as_bytes()).unwrap();
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
        let service_name = format!("{}Service", type_name);

        let mut mht_ctx = mhtemplate::Context::new();
        mht_ctx["PACKAGE_NAME"] = module_name.to_lowercase();
        mht_ctx["TYPE_NAME"] = type_name;
        mht_ctx["ID_TYPE_NAME"] = id_type_name.to_owned();
        mht_ctx["ID_TYPE_PRIMITIVE"] = id_type_primitive;
        mht_ctx["SERVICE_NAME"] = service_name.to_owned();

        // filename: ./<module>/<service_name>.go
        let service_tpl = mhtemplate::TemplateFactory::new(
            "package {{$PACKAGE_NAME}}\n\
            \n\
            // {{$SERVICE_NAME}} is the contract for a service related to {{$TYPE_NAME}}.\n\
            type {{$SERVICE_NAME}} struct {\n\
            \t//TODO: implement this.\n\
            }\n\
            \n",
        )
        .parse()
        .unwrap();

        let service_code = service_tpl.evaluate(&mut mht_ctx).unwrap();
        fs::create_dir_all(format!("{}/{}", base_dir, module_name,)).unwrap();
        let mut service_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(format!("{}/{}/{}.go", base_dir, module_name, service_name,))
            .unwrap();
        service_file.write_all(service_code.as_bytes()).unwrap();

        Ok(())
    }

    fn generate_value_object_codes(
        &self,
        module_name: &String,
        vo: &value_object::ValueObject,
        identifier: &String,
    ) -> Result<(), Box<dyn error::Error>> {
        let base_dir = &self.base_dir;
        let mut mht_ctx = mhtemplate::Context::new();
        mht_ctx["PACKAGE_NAME"] = module_name.to_lowercase();
        mht_ctx["TYPE_NAME"] = identifier.to_owned();

        let tpl: Box<dyn mhtemplate::Template>;

        use data_type::DataType;
        match vo.data_type {
            DataType::Struct => {
                tpl = mhtemplate::TemplateFactory::new(
                    "package {{$PACKAGE_NAME}}\n\
                    \n\
                    // {{$TYPE_NAME}} is ....\n\
                    type {{$TYPE_NAME}} struct {\n\
                    \t// TODO
                    }\n\
                    \n",
                )
                .parse()
                .unwrap();
            }
            _ => {
                tpl = mhtemplate::TemplateFactory::new(
                    "package {{$PACKAGE_NAME}}\n\
                    \n\
                    // {{$TYPE_NAME}} is ....\n\
                    type {{$TYPE_NAME}} {{$PRIMITIVE_TYPE_NAME}}\n\
                    \n",
                )
                .parse()
                .unwrap();

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

        let service_code = tpl.evaluate(&mut mht_ctx).unwrap();
        fs::create_dir_all(format!("{}/{}", base_dir, module_name,)).unwrap();
        let mut service_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(format!("{}/{}/{}.go", base_dir, module_name, identifier,))
            .unwrap();
        service_file.write_all(service_code.as_bytes()).unwrap();

        Ok(())
    }
}

impl codegen::CodeGenerator for GoCodeGenerator {
    fn generate_module_codes(
        &self,
        module_name: &String,
        module_def: &module::ModuleDefinition,
    ) -> Result<(), Box<dyn error::Error>> {
        for symbol in &module_def.symbols {
            let params = &symbol.parameters;
            if let Some(ent) = params.downcast_ref::<entity::Entity>() {
                self.generate_entity_codes(module_name, ent, &symbol.identifier)
                    .unwrap();
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
                    )
                    .unwrap();
                    continue;
                }
                continue;
            }
            if let Some(vo) = params.downcast_ref::<value_object::ValueObject>() {
                self.generate_value_object_codes(module_name, vo, &symbol.identifier)
                    .unwrap();
                continue;
            }
        }
        Ok(())
    }
}
