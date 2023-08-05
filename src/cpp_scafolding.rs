use super::*;

pub struct CPPProjectScafolding{
    project_name: String,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

 impl CPPProjectScafolding{

    pub fn new(project_name: String) -> CPPProjectScafolding{
        Self{project_name}
    }

    pub fn generate_project_scafolding(&self) -> Result<()>  {
        let _ = make_project_dir(&self.project_name);
        make_project_dir(&format!("{}{}", self.project_name, "/src"))?;
        make_project_dir(&format!("{}{}", self.project_name, "/include"))?;
        make_project_dir(&format!("{}{}", self.project_name, "/tests"))?;
        make_project_dir(&format!("{}{}", self.project_name, "/cmake"))?;
        make_project_dir(&format!("{}{}", self.project_name, "/docs"))?;

        let _ = make_defaults(&self.project_name);
        Ok(())    
    }

 }