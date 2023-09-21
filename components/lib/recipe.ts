import { Hit } from "@/src-tauri/bindings/Hit";
import { RecipeData } from "@/src-tauri/bindings/RecipeData";
import { RecipeDetails } from "@/src-tauri/bindings/RecipeDetails";

export function processRecipeData(edamamHit: Hit, recipeScrapersData: RecipeData): RecipeDetails {
  return {
    image: edamamHit.recipe.image,
    url: edamamHit.recipe.url,
    servings: convertToInteger(edamamHit.recipe.yield),
    time: convertToInteger(edamamHit.recipe.totalTime),
    calories: convertToInteger(edamamHit.recipe.calories),
    source: recipeScrapersData.host,
    ingredients: recipeScrapersData.ingredients,
    instructions: recipeScrapersData.instructions_list,
    title: edamamHit.recipe.label,
    id: null,
    date_added: null,
  };
}

function convertToInteger(floatValue: number) {
  return Math.round(floatValue);
}
