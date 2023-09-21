import { useFetchData } from "@/components/db/get";
import ErrorMessage from "@/components/error";
import Spinner from "@/components/spinner";
import Table from "@/components/table";
import { RecipeDetails } from "@/src-tauri/bindings/RecipeDetails";
import { Flex, Text } from "@chakra-ui/react";

const Favorites = () => {
  const { data, error, isLoading } = useFetchData<RecipeDetails[]>({
    endpoint: "get_all_recipes",
    param: {},
  });

  return (
    <>
      <Flex w="100%" m={4} flexDirection="column">
        <Text fontWeight="extrabold" fontSize="2xl" fontFamily="mono" mb={4}>
          Saved Recipes
        </Text>
        {data && <Table recipes={data} />}
        <Flex w="100%" justify="center" alignItems="center">
          {error && (
            <ErrorMessage message="There was an error fetching your recipes database, please try again later or submit an issue on Github" />
          )}
          {isLoading && <Spinner message="Retrieving results from recipe database" />}
        </Flex>
      </Flex>
    </>
  );
};

export default Favorites;
