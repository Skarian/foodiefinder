import type { Hit } from "@/src-tauri/bindings/Hit";
import { Box, Flex, Text } from "@chakra-ui/react";
import NextLink from "next/link";

const Card = ({ data }: { data: Hit }) => {
  const placeholder = "https://placehold.co/600x400?text=No+Image+Preview+Available";
  const getImageSrc = () => {
    const priorityList = [
      data.recipe.images.LARGE?.url,
      data.recipe.images.REGULAR?.url,
      data.recipe.images.SMALL?.url,
      data.recipe.images.THUMBNAIL?.url,
    ];

    for (const src of priorityList) {
      if (src) {
        return src;
      }
    }

    return placeholder;
  };

  const imageSrc = getImageSrc();

  const handleImageError = (event: React.SyntheticEvent<HTMLImageElement, Event>) => {
    event.currentTarget.src = placeholder;
  };

  const hitData = encodeURIComponent(JSON.stringify(data));

  return (
    <NextLink
      href={{ pathname: "/recipe", query: { hitData: hitData, action: "search", url: data.recipe.url } }}
      passHref
    >
      <Box
        borderWidth="1px"
        borderRadius="md"
        overflow="hidden"
        position="relative"
        transition="box-shadow 0.3s"
        _hover={{ boxShadow: "md" }}
        w="200px"
        h="200px"
      >
        <Box as="img" src={imageSrc} alt="Card Image" w="100%" h="100%" objectFit="cover" onError={handleImageError} />

        <Flex
          align="center"
          justify="center"
          bg="rgba(0, 0, 0, 0.7)"
          color="white"
          position="absolute"
          top="0"
          left="0"
          w="100%"
          h="100%"
          opacity="0"
          transition="opacity 0.3s"
          _hover={{ opacity: "1" }}
        >
          <Text fontSize="xl" textAlign="center" fontFamily="mono" m={5} noOfLines={5}>
            {data.recipe.label}
          </Text>
        </Flex>
      </Box>
    </NextLink>
  );
};

export default Card;
