use handlebars::Handlebars;
use std::collections::BTreeMap;

pub fn get_readme(project_name: &str) -> Result<String, Box<dyn std::error::Error>>{
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("readme_template", DEFAULT_README)?;

    let mut data = BTreeMap::new();
    data.insert("project_name".to_string(), project_name.to_string());
    let rslt = handlebars.render("readme_template", &data)?;
    return Ok(rslt);
}


pub const DEFAULT_README: &str = r#"

# {{project_name}} )

## TL;DR

## Prerequisites

## Configuration

## Build Instructions

## Build Instructions
"#;

