#[derive(Debug, Clone, PartialEq)]
pub enum FormField {
    Name,
    Description,
    Tags,
    Submit,
}

#[derive(Debug, Clone)]
pub struct ProjectFormState {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub current_field: FormField,
    pub validation_errors: Vec<String>,
    pub tag_input: String, // For adding new tags
}

impl Default for ProjectFormState {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            tags: Vec::new(),
            current_field: FormField::Name,
            validation_errors: Vec::new(),
            tag_input: String::new(),
        }
    }
}

impl ProjectFormState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate(&mut self) -> bool {
        self.validation_errors.clear();

        if self.name.trim().is_empty() {
            self.validation_errors.push("Project name is required".to_string());
        } else if self.name.len() > 100 {
            self.validation_errors.push("Project name must be 100 characters or less".to_string());
        }

        if self.description.len() > 500 {
            self.validation_errors.push("Description must be 500 characters or less".to_string());
        }

        self.validation_errors.is_empty()
    }

    pub fn is_valid(&self) -> bool {
        !self.name.trim().is_empty() 
            && self.name.len() <= 100 
            && self.description.len() <= 500
    }

    pub fn add_tag(&mut self) {
        let tag = self.tag_input.trim().to_string();
        if !tag.is_empty() && !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.tag_input.clear();
        }
    }

    pub fn remove_tag(&mut self, index: usize) {
        if index < self.tags.len() {
            self.tags.remove(index);
        }
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn next_field(&mut self) {
        self.current_field = match self.current_field {
            FormField::Name => FormField::Description,
            FormField::Description => FormField::Tags,
            FormField::Tags => FormField::Submit,
            FormField::Submit => FormField::Name,
        };
    }

    pub fn previous_field(&mut self) {
        self.current_field = match self.current_field {
            FormField::Name => FormField::Submit,
            FormField::Description => FormField::Name,
            FormField::Tags => FormField::Description,
            FormField::Submit => FormField::Tags,
        };
    }
}