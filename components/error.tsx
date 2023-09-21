import { Spinner as ChakraSpinner, Text } from "@chakra-ui/react";

interface ErrorProps {
  message: string;
}

const ErrorMessage = ({ message }: ErrorProps) => {
  return (
    <>
      <Text as="b" color="red" fontFamily="mono" textAlign="center" maxWidth="lg">
        {message}
      </Text>
    </>
  );
};

export default ErrorMessage;
