import { UseToastOptions } from "@chakra-ui/react";

const DURATION = 5000;

type Variants = "success" | "error";

export const addRecipeToast = (variant: Variants): UseToastOptions => {
  return {
    title: variant === "success" ? "Recipe Added to Favorites!" : "Unable to Add Recipe to Favorites",
    description: variant === "success" ? "Saved recipe successfully to database" : "Unable to save recipe to database",
    status: variant === "success" ? "success" : "error",
    duration: DURATION,
    isClosable: true,
  };
};

export const deleteRecipeToast = (variant: Variants): UseToastOptions => {
  return {
    title: variant === "success" ? "Deleted recipe!" : "Unable to Delete Recipe!",
    description:
      variant === "success" ? "Deleted recipe successfully from database" : "Unable to delete recipe from database",
    status: variant === "success" ? "success" : "error",
    duration: DURATION,
    isClosable: true,
  };
};

export const updateRecipeToast = (variant: Variants): UseToastOptions => {
  return {
    title: variant === "success" ? "Updated recipe!" : "Unable to Update Recipe!",
    description:
      variant === "success" ? "Updated recipe successfully in database" : "Unable to update recipe in database",
    status: variant === "success" ? "success" : "error",
    duration: DURATION,
    isClosable: true,
  };
};
