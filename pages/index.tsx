import { SearchAction, SearchActionType } from "@/src-tauri/bindings/Query";
import { SearchIcon } from "@chakra-ui/icons";
import {
  Flex,
  Input,
  InputGroup,
  InputLeftElement,
  Text,
  Kbd,
  FormControl,
  FormLabel,
  FormErrorMessage,
} from "@chakra-ui/react";
import { useRouter } from "next/router";
import { useForm } from "react-hook-form";

export default function Home() {
  const router = useRouter();

  interface FormValues {
    search: string;
  }

  const {
    handleSubmit,
    register,
    formState: { errors },
  } = useForm<FormValues>();

  function onSubmit(values: FormValues): void {
    const data = { query: values.search };

    const action: SearchAction = {
      type: SearchActionType.Original,
    };

    router.push({
      pathname: "/search",
      query: {
        action: JSON.stringify(action),
        searchTerm: data.query,
      },
    });
  }

  return (
    <>
      <Flex direction="column" justify="center" align="center" w="100%" gap={10}>
        <Text fontSize="2xl" fontFamily="mono" noOfLines={1} fontWeight="extrabold" userSelect="none">
          What do you want to cook today?
        </Text>
        <form onSubmit={handleSubmit(onSubmit)}>
          <FormControl isInvalid={Boolean(errors?.search)}>
            <FormLabel htmlFor="search" display="none">
              Search Query
            </FormLabel>
            <InputGroup width="350px">
              <InputLeftElement pointerEvents="none">
                <SearchIcon color="gray.300" />
              </InputLeftElement>
              <Input
                id="search"
                placeholder="ex: Spaghetti, Cuban Sandwich"
                fontFamily="mono"
                borderColor="gray.500"
                {...register("search", {
                  required: "This is required",
                  minLength: {
                    value: 1,
                    message: "Minimum length should be 1 character",
                  },
                })}
              />
            </InputGroup>
            <FormErrorMessage>
              {errors?.search && (
                <>
                  {typeof errors.search === "string" ? (
                    <Text userSelect="none" fontFamily="mono">
                      {errors.search}
                    </Text>
                  ) : (
                    <Text userSelect="none" fontFamily="mono">
                      {errors.search.message as React.ReactNode}
                    </Text>
                  )}
                </>
              )}
            </FormErrorMessage>
          </FormControl>
        </form>
        <Text fontFamily="mono" noOfLines={1} textColor="gray.500" userSelect="none">
          Press <Kbd>enter</Kbd> to search!
        </Text>
      </Flex>
    </>
  );
}
