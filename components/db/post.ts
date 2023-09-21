import { RecipeDetails } from "@/src-tauri/bindings/RecipeDetails";
import { invoke } from "@/components/lib/tauri";
import { addRecipeToast, deleteRecipeToast, updateRecipeToast } from "@/components/lib/toasts";
import { createStandaloneToast } from "@chakra-ui/react";

const { toast } = createStandaloneToast();

export const updateRecipe = (recipe: RecipeDetails) => {
  invoke("update_recipe", { recipe: recipe })
    .then(() => {
      toast(updateRecipeToast("success"));
      console.log("Request successfully made");
    })
    .catch((e) => {
      toast(addRecipeToast("error"));
      console.error(e);
    });
};

export const addRecipe = (recipe: RecipeDetails) => {
  invoke("add_recipe", { recipe: recipe })
    .then(() => {
      toast(addRecipeToast("success"));
      console.log("Request successfully made");
    })
    .catch((e) => {
      toast(addRecipeToast("error"));
      console.error(e);
    });
};

export const deleteRecipe = (id: number) => {
  invoke("delete_recipe_by_id", { id: id })
    .then(() => {
      toast(deleteRecipeToast("success"));
      console.log("Request successfully made");
    })
    .catch((e) => {
      toast(deleteRecipeToast("error"));
      console.error(e);
    });
};
