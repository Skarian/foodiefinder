import { useRouter } from "next/router";
import { RecipeDetails } from "@/src-tauri/bindings/RecipeDetails";
import { ChatIcon, EditIcon, ExternalLinkIcon } from "@chakra-ui/icons";
import {
  Button,
  ButtonGroup,
  Flex,
  HStack,
  Icon,
  Image,
  Link,
  Stack,
  Tag,
  TagLabel,
  Text,
  Tooltip,
} from "@chakra-ui/react";
import { useEffect, useState } from "react";
import { AiFillSave } from "react-icons/ai";
import { useFetchData } from "@/components/db/get";
import { addRecipe, deleteRecipe } from "@/components/db/post";

interface HeaderProps {
  imgSrc: string;
  recipeName: string;
  servings: number;
  time: number;
  calories: number;
  url: string;
  host: string | null;
  recipe: RecipeDetails;
  action: "search" | "saved";
  openModal: () => void;
}
const RecipeHeader = ({
  imgSrc,
  recipeName,
  servings,
  time,
  calories,
  url,
  host,
  recipe,
  action,
  openModal,
}: HeaderProps) => {
  const [saved, setSaved] = useState(false);
  const handleSave = () => {
    addRecipe(recipe);
    setSaved(true);
  };

  const router = useRouter();

  const handleDelete = () => {
    if (recipe.id) {
      deleteRecipe(recipe.id);
      router.push("/favorites");
    }
    console.log("They want to delete this bad boy");
  };

  const { data: savedInDB } = useFetchData<boolean>({
    endpoint: "does_recipe_exist_by_url",
    param: { url: url },
  });

  useEffect(() => {
    if (!saved && savedInDB) {
      setSaved(true);
    }
  }, [savedInDB, saved]);

  return (
    <>
      {/* Picture and info */}
      <Flex
        justify="center"
        align="center"
        flexDirection={{ base: "column", md: "row" }}
        maxW={{ base: "md", sm: "lg", md: "3xl" }}
      >
        {/* Picture */}
        <Flex justify="center">
          <Image
            aspectRatio={1}
            objectFit="cover"
            borderRadius="lg"
            w={{ base: "xs", md: "md" }}
            src={imgSrc}
            alt={recipeName}
            mb={{ base: 4, md: 0 }}
            fallbackSrc="https://placehold.co/600x400?text=Image+Loading"
          />
        </Flex>
        {/* Info */}
        <Flex ml={{ base: 0, md: 8 }} justify="center" align="center" minW="sm">
          <Stack spacing={4}>
            {/* Recipe Title */}
            <Text
              fontSize={{ base: "2xl", md: "4xl" }}
              noOfLines={2}
              fontFamily="mono"
              textAlign={{ base: "center", md: "left" }}
            >
              {recipeName}
            </Text>
            {/* Servings, Time, and Calories */}
            <HStack spacing={2} flexWrap="wrap" justify={{ base: "center", md: "initial" }}>
              {/* Servings */}
              {servings > 0 && (
                <Tag size={{ base: "xs", sm: "sm", md: "md" }} borderRadius="full" variant="subtle" colorScheme="green">
                  <TagLabel fontFamily="mono" fontSize={{ base: "xs", sm: "sm", md: "md" }}>
                    {servings} Servings
                  </TagLabel>
                </Tag>
              )}
              {/* Time */}
              {time > 0 && (
                <Tag size={{ base: "xs", sm: "sm", md: "md" }} borderRadius="full" variant="subtle" colorScheme="cyan">
                  <TagLabel fontFamily="mono" fontSize={{ base: "xs", sm: "sm", md: "md" }}>
                    {time} mins
                  </TagLabel>
                </Tag>
              )}
              {/* Calories */}
              {calories > 0 && (
                <Tag
                  size={{ base: "xs", sm: "sm", md: "md" }}
                  borderRadius="full"
                  variant="subtle"
                  colorScheme="orange"
                >
                  <TagLabel fontFamily="mono" fontSize={{ base: "xs", sm: "sm", md: "md" }}>
                    {calories} Cal
                  </TagLabel>
                </Tag>
              )}
            </HStack>
            {/* Save Recipe and additional buttons */}
            <Flex justify={{ base: "center", md: "inherit" }}>
              <ButtonGroup>
                {action === "search" &&
                  (saved ? (
                    <Tooltip
                      shouldWrapChildren
                      label="You already saved it, check favorites!"
                      aria-label="You already saved it, check favorites!"
                    >
                      <Button
                        colorScheme="yellow"
                        variant="solid"
                        isDisabled={saved ? true : false}
                        fontFamily="mono"
                        leftIcon={<Icon as={AiFillSave} fontSize="large" />}
                        size="sm"
                      >
                        Save Recipe
                      </Button>
                    </Tooltip>
                  ) : (
                    <Button
                      colorScheme="yellow"
                      variant="solid"
                      fontFamily="mono"
                      leftIcon={<Icon as={AiFillSave} fontSize="large" />}
                      size="sm"
                      onClick={handleSave}
                    >
                      Save Recipe
                    </Button>
                  ))}

                {action === "saved" && (
                  <>
                    {/* <Button
                      colorScheme="yellow"
                      fontFamily="mono"
                      size="sm"
                      leftIcon={<EditIcon />}
                      onClick={openModal}
                    >
                      Edit
                    </Button>
                    <Button colorScheme="green" fontFamily="mono" leftIcon={<ChatIcon />} size="sm">
                      AI Enhance
                    </Button> */}
                    <Button
                      colorScheme="red"
                      fontFamily="mono"
                      size="sm"
                      leftIcon={<EditIcon />}
                      onClick={handleDelete}
                    >
                      Delete
                    </Button>
                  </>
                )}
              </ButtonGroup>
            </Flex>
            {/* Link to original URL */}
            <Link href={url} isExternal textAlign={{ base: "center", md: "left" }}>
              <Button size="sm" variant="link" rightIcon={<ExternalLinkIcon />}>
                <Text fontFamily="mono" as="u">
                  {host ? `View original recipe on ${host}` : "View original recipe"}
                </Text>
              </Button>
            </Link>
          </Stack>
        </Flex>
      </Flex>
    </>
  );
};

export default RecipeHeader;
