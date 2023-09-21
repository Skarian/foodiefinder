import { Flex, Text } from "@chakra-ui/react";
import { FC } from "react";

const Footer: FC = () => {
  return (
    <Flex direction="row" w="100%" justify="center">
      <Flex alignItems="center">
        <Text
          fontSize="sm"
          color="white"
          fontWeight="bold"
          fontFamily="mono"
          noOfLines={1}
          userSelect="none"
        >
          Made with ♥️ by Neil Skaria
        </Text>
      </Flex>
    </Flex>
  );
};

export default Footer;
