import { RecipeDetails } from "@/src-tauri/bindings/RecipeDetails";
import {
  TableContainer,
  Button,
  TableCaption,
  Thead,
  Tr,
  Th,
  Tbody,
  Td,
  Tfoot,
  Table as ChakraTable,
  Text,
  Flex,
} from "@chakra-ui/react";
import { Link as ChakraLink } from "@chakra-ui/react";
import { useRouter } from "next/router";
import React from "react";

interface TableProps {
  recipes: RecipeDetails[];
}

const Table = ({ recipes }: TableProps) => {
  const router = useRouter();
  return (
    <>
      {recipes.length > 0 ? (
        <TableContainer overflowY="auto" overflowX="scroll">
          <ChakraTable variant="striped" fontFamily="mono">
            <TableCaption fontFamily="mono">Coming soon: Adding your own custom recipes directly!</TableCaption>
            <Tbody>
              {recipes.map((recipe) => {
                const recipeDetails = encodeURIComponent(JSON.stringify(recipe));
                return (
                  <Tr key={recipe.id}>
                    <Td>
                      <Text isTruncated maxW={{ base: "3xs", sm: "xs", md: "md", lg: "lg" }}>
                        {recipe.title}
                      </Text>
                    </Td>
                    <Td display={{ base: "none", md: "table-cell" }}>{recipe.date_added?.split(" ")[0]}</Td>
                    <Td>
                      <Button
                        onClick={() => {
                          router.push({
                            pathname: "/recipe",
                            query: {
                              action: "saved",
                              savedRecipe: recipeDetails,
                            },
                          });
                        }}
                        variant="solid"
                        size="sm"
                        colorScheme="orange"
                        fontWeight="normal"
                      >
                        View
                      </Button>
                    </Td>
                  </Tr>
                );
              })}
            </Tbody>
          </ChakraTable>
        </TableContainer>
      ) : (
        <>
          <Flex w="100%" h="100%" alignItems="center" justify="center">
            <Text textAlign="center" maxW="lg" fontFamily="mono">
              Looks like you have not saved any recipes yet. Checkout the search page to find some!
            </Text>
          </Flex>
        </>
      )}
    </>
  );
};

export default Table;
