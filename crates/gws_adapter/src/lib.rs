use ui_contracts::{AutomationRecipe, RecipePreview};

#[must_use]
pub fn preview_recipe(recipe: &AutomationRecipe) -> RecipePreview {
    let mut scopes = Vec::new();
    let mut commands = Vec::new();

    for step in &recipe.steps {
        match step.step_type.as_str() {
            "gws.chat.spaces.messages.create" => {
                scopes.push("chat.messages.create".to_string());
                commands.push(format!(
                    "gws chat spaces messages create --parent {} --dry-run",
                    step.parent.as_deref().unwrap_or("spaces/UNKNOWN")
                ));
            }
            "gws.gmail.users.messages.send" => {
                scopes.push("gmail.send".to_string());
                commands.push("gws gmail users messages send --dry-run".to_string());
            }
            "gemini.summarize" => {
                scopes.push("gemini.generate".to_string());
                commands.push("weave gemini summarize --dry-run".to_string());
            }
            _ => commands.push(format!("weave action {} --dry-run", step.step_type)),
        }
    }

    RecipePreview {
        id: recipe.id.clone(),
        name: recipe.name.clone(),
        command_preview: commands.join(" && "),
        payload_preview: format!("{} step(s) will run against {}", recipe.steps.len(), recipe.trigger),
        required_scopes: scopes,
    }
}
