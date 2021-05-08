//

use std::io;

use azml::azml::{
    adjunct::{adjunct, adjunct_entity},
    entity::{abstract_, root_entity},
    error, module,
};

pub fn write_dot(
    w: &mut impl io::Write,
    module_name: String,
    module_def: &module::ModuleDefinition,
) -> Result<(), error::Error> {
    w.write(format!("digraph {} {{\n", module_name).as_bytes())?;
    for symbol in &module_def.symbols {
        let params = &symbol.definition;
        if let Some(ent) = params.downcast_ref::<root_entity::RootEntity>() {
            ent.write_dot_identifier(w, symbol.identifier.clone())?;
        } else if let Some(adj) = params.downcast_ref::<adjunct::Adjunct>() {
            adj.write_dot_identifier(w, symbol.identifier.clone())?;
        } else if let Some(abst) = params.downcast_ref::<abstract_::Abstract>() {
            abst.write_dot_identifier(w, symbol.identifier.clone())?;
        }
    }
    w.write_all(b"\n")?;
    for symbol in &module_def.symbols {
        let params = &symbol.definition;
        if let Some(ent) = params.downcast_ref::<root_entity::RootEntity>() {
            ent.write_dot_relationships(w, symbol.identifier.clone())?;
        } else if let Some(adj) = params.downcast_ref::<adjunct::Adjunct>() {
            adj.write_dot_relationships(w, symbol.identifier.clone())?;
        } else if let Some(abst) = params.downcast_ref::<abstract_::Abstract>() {
            abst.write_dot_relationships(w, symbol.identifier.clone())?;
        }
    }
    w.write_all(b"}\n")?;

    Ok(())
}

trait DotNode {
    fn write_dot_identifier(
        &self,
        w: &mut impl io::Write,
        identifier: String,
    ) -> Result<(), io::Error>;
    fn write_dot_relationships(
        &self,
        w: &mut impl io::Write,
        identifier: String,
    ) -> Result<(), io::Error>;
}

impl DotNode for adjunct::Adjunct {
    fn write_dot_identifier(
        &self,
        w: &mut impl io::Write,
        identifier: String,
    ) -> Result<(), io::Error> {
        if let Some(_) = self
            .definition
            .downcast_ref::<adjunct_entity::AdjunctEntity>()
        {
            w.write(
                format!(
                    "  {} [shape=ellipse style=filled color=lightskyblue1]\n",
                    identifier
                )
                .as_bytes(),
            )?;
        } else {
            w.write(
                format!(
                    "  {} [shape=ellipse style=filled color=darkolivegreen1]\n",
                    identifier
                )
                .as_bytes(),
            )?;
        }
        Ok(())
    }
    fn write_dot_relationships(
        &self,
        w: &mut impl io::Write,
        identifier: String,
    ) -> Result<(), io::Error> {
        for ent in &self.hosts {
            let kind_str = String::from(&ent.kind);
            w.write(
                format!(
                    "  {} -> {}\n",
                    identifier,
                    if kind_str.contains(".") {
                        format!("\"{}\"", kind_str)
                    } else {
                        kind_str.to_owned()
                    }
                )
                .as_bytes(),
            )?;
        }
        Ok(())
    }
}

impl DotNode for root_entity::RootEntity {
    fn write_dot_identifier(
        &self,
        w: &mut impl io::Write,
        identifier: String,
    ) -> Result<(), io::Error> {
        w.write(
            format!(
                "  {} [shape=rect color=lightskyblue1 style=filled]\n",
                identifier
            )
            .as_bytes(),
        )?;
        Ok(())
    }
    fn write_dot_relationships(
        &self,
        _w: &mut impl io::Write,
        _identifier: String,
    ) -> Result<(), io::Error> {
        // Do nothing
        Ok(())
    }
}

impl DotNode for abstract_::Abstract {
    fn write_dot_identifier(
        &self,
        w: &mut impl io::Write,
        identifier: String,
    ) -> Result<(), io::Error> {
        w.write(
            format!(
                "  {} [shape=rect color=lightskyblue1 style=dashed]\n",
                identifier,
            )
            .as_bytes(),
        )?;
        Ok(())
    }
    fn write_dot_relationships(
        &self,
        _w: &mut impl io::Write,
        _identifier: String,
    ) -> Result<(), io::Error> {
        // Do nothing
        Ok(())
    }
}
