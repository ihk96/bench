use tui_input::Input;


#[derive(Debug, Clone)]
pub struct Settings {
    pub host: Input,
    pub model_name: Input,
    pub api_key: Input,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            host: Input::new("".to_string()),
            model_name: Input::new("".to_string()),
            api_key: Input::new("".to_string()),
        }
    }
}

