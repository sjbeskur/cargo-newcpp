use handlebars::Handlebars;
use std::collections::BTreeMap;

pub fn get_cmake(project_name: &str, template: &str) -> Result<String, Box<dyn std::error::Error>>{
    let mut handlebars = Handlebars::new();

    handlebars.register_template_string("cmake_template", template)?;

    let mut data = BTreeMap::new();
    data.insert("project_name".to_string(), project_name.to_string());
    let rslt = handlebars.render("cmake_template", &data)?;
    Ok(rslt)
}
