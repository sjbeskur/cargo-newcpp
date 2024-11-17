use handlebars::Handlebars;
use std::collections::BTreeMap;

pub fn get_readme(project_name: &str, template: &str) -> Result<String, Box<dyn std::error::Error>>{
    let mut handlebars = Handlebars::new();

    handlebars.register_template_string("readme_template", template)?;

    let mut data = BTreeMap::new();
    data.insert("project_name".to_string(), project_name.to_string());
    let rslt = handlebars.render("readme_template", &data)?;
    Ok(rslt)
}



