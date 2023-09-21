import { Spinner as ChakraSpinner, Text, Stack } from "@chakra-ui/react";

interface SpinnerProps {
  message: string;
}

const Spinner = ({ message }: SpinnerProps) => {
  return (
    <>
      <Stack alignItems="center">
        <ChakraSpinner mb={4} size="xl" color="twitter.500" thickness="4px" />
        <Text textAlign="center" as="i" fontFamily="mono">
          {message}
        </Text>
      </Stack>
    </>
  );
};

export default Spinner;
