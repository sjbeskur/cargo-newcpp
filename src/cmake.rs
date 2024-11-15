use handlebars::Handlebars;
use std::collections::BTreeMap;

pub fn get_cmake(project_name: &str) -> Result<String, Box<dyn std::error::Error>>{
    let mut handlebars = Handlebars::new();
    let exe_template = include_str!("../templates/CMakeLists_exe.in");

    handlebars.register_template_string("cmake_template", exe_template)?;

    let mut data = BTreeMap::new();
    data.insert("project_name".to_string(), project_name.to_string());
    let rslt = handlebars.render("cmake_template", &data)?;
    Ok(rslt)
}
