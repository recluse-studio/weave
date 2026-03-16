use anyhow::{Result, anyhow};
use gws_adapter::preview_recipe;
use ui_contracts::{AutomationRecipe, RecipePreview};
use workspace::WorkspaceRepository;

pub fn list_recipes(repository: &WorkspaceRepository) -> Result<Vec<AutomationRecipe>> {
    Ok(repository.load_snapshot()?.automations)
}

pub fn preview(repository: &WorkspaceRepository, recipe_id: &str) -> Result<RecipePreview> {
    let recipe = repository
        .load_snapshot()?
        .automations
        .into_iter()
        .find(|recipe| recipe.id == recipe_id)
        .ok_or_else(|| anyhow!("recipe not found: {recipe_id}"))?;

    Ok(preview_recipe(&recipe))
}
