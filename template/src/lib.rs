use act_sdk::prelude::*;

#[act_component(
    name = "{{ project_name }}",
    version = "0.1.0",
    description = "{{ description }}",
)]
mod component {
    use super::*;

    #[act_tool(description = "Say hello", read_only)]
    fn hello(
        /// Name to greet
        name: Option<String>,
    ) -> ActResult<String> {
        let who = name.unwrap_or_else(|| "world".to_string());
        Ok(format!("Hello, {who}!"))
    }
}
