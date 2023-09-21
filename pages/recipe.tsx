import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { invoke } from "@/components/lib/tauri";
import { RecipeData } from "@/src-tauri/bindings/RecipeData";
import { Hit } from "@/src-tauri/bindings/Hit";
import { Flex, Stack, useDisclosure } from "@chakra-ui/react";
import RecipeHeader from "@/components/recipe/header";
import RecipeAccordian from "@/components/recipe/accordian";
import Spinner from "@/components/spinner";
import { RecipeDetails } from "@/src-tauri/bindings/RecipeDetails";
import { processRecipeData } from "@/components/lib/recipe";
import { getRightImage } from "@/components/lib/image";
import { ParsedUrlQuery } from "querystring";
import ErrorMessage from "@/components/error";
import EditModal from "@/components/edit";

type PageAction = "search" | "saved";

interface QueryParams extends ParsedUrlQuery {
  action: string;
  hitData?: string;
  savedRecipe?: string;
  url?: string;
}

const Recipe = () => {
  // Query data
  const router = useRouter();
  const { hitData, action, savedRecipe, url } = router.query as QueryParams;
  const pageAction = action as PageAction;

  // Component State
  const [errorStatus, setErrorStatus] = useState(false);
  const [recipeDetails, setRecipeDetails] = useState<RecipeDetails | null>(null);
  const [image, setImage] = useState<string>("");
  const { isOpen, onOpen, onClose } = useDisclosure();

  // Processes data if required and sets state used in UI
  useEffect(() => {
    if (hitData && url && pageAction === "search") {
      const decodedData = decodeURIComponent(hitData);
      const edamamData: Hit = JSON.parse(decodedData) as Hit;
      setImage(edamamData.recipe.image);
      invoke("get_recipe_details", { url: url })
        .then((value: unknown) => {
          const scraped_data = value as RecipeData;
          const recipeDetails = processRecipeData(edamamData, scraped_data);
          setRecipeDetails(recipeDetails);
        })
        .catch((err) => {
          setErrorStatus(true);
          console.log(`Error: ${err}`);
        });
    }

    if (savedRecipe && pageAction === "saved") {
      const decodedData = decodeURIComponent(savedRecipe);
      const parsedData: RecipeDetails = JSON.parse(decodedData) as RecipeDetails;
      setRecipeDetails(parsedData);
      const image = getRightImage(parsedData.image);
      setImage(image);
    }
  }, [hitData, savedRecipe, pageAction, url]);

  return (
    <>
      {pageAction === "saved" && recipeDetails && (
        <EditModal isOpen={isOpen} onClose={onClose} recipe={recipeDetails} />
      )}
      {recipeDetails && (
        <Stack w="100%" alignItems="center" p={4}>
          <RecipeHeader
            imgSrc={image}
            recipeName={recipeDetails.title}
            servings={recipeDetails.servings}
            time={recipeDetails.time}
            calories={recipeDetails.calories}
            url={recipeDetails.url}
            host={recipeDetails.source}
            recipe={recipeDetails}
            action={pageAction}
            openModal={onOpen}
          />
          <Flex justify="center" pt={10}>
            <Flex
              w="100%"
              flexDirection={{ base: "column", md: "row" }}
              justify="space-evenly"
              alignItems={{ base: "center", md: "inherit" }}
            >
              <Flex w={{ base: "80%", md: "45%" }}>
                <RecipeAccordian title="Ingredients List" content={recipeDetails.ingredients} />
              </Flex>
              <Flex w={{ base: "80%", md: "45%" }}>
                <RecipeAccordian title="Instructions" content={recipeDetails.instructions} />
              </Flex>
            </Flex>
          </Flex>
        </Stack>
      )}
      {!recipeDetails && !errorStatus && (
        <Flex w="100%" h="100%" justify="center" alignItems="center" flexDirection="column">
          <Spinner message="Scraping website, may take a few moments" />
        </Flex>
      )}
      {!recipeDetails && errorStatus && (
        <Flex w="100%" h="100%" justify="center" alignItems="center" flexDirection="column">
          <ErrorMessage message="There was an issue fetching the recipe, please try again later or submit an issue on Github" />
        </Flex>
      )}
    </>
  );
};

export default Recipe;
