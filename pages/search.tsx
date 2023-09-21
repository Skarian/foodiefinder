import { useRouter } from "next/router";
import { useEffect, useMemo } from "react";
import type { EdamamResponse } from "@/src-tauri/bindings/EdamamResponse";
import { Flex, Text } from "@chakra-ui/react";
import { SearchAction, SearchActionType } from "@/src-tauri/bindings/Query";
import { invoke } from "@/components/lib/tauri";
import SearchHeader from "@/components/search/header";
import SearchResults from "@/components/search/results";
import Spinner from "@/components/spinner";
import useSWR, { preload } from "swr";

interface FetcherParam {
  endpoint: string;
  param: { query?: string; nextUrl?: string };
}

const fetcher = ([{ endpoint, param }]: FetcherParam[]): Promise<EdamamResponse> => {
  return invoke(endpoint, param)
    .then((value: unknown) => {
      return value as EdamamResponse;
    })
    .catch((error: Error) => {
      console.error(error);
      throw error;
    });
};

const Search = () => {
  const router = useRouter();
  const excludeScrapable = true;
  const query = router.query;
  const searchTerm = query.searchTerm;

  const action = useMemo(() => {
    return query.action ? (JSON.parse(query.action as string) as SearchAction) : undefined;
  }, [query.action]);

  const goToNext = (nextUrl: string) => {
    const action: SearchAction = {
      type: SearchActionType.Next,
      nextUrl: nextUrl,
    };

    router.push({
      pathname: "/search",
      query: {
        action: JSON.stringify(action),
        searchTerm: searchTerm,
      },
    });
  };

  const { data: searchResults, error } = useSWR<EdamamResponse, Error>(
    action && action.type === SearchActionType.Original
      ? [{ endpoint: "search_recipes", param: { query: searchTerm } }]
      : [{ endpoint: "get_next_recipes", param: { nextUrl: action?.nextUrl } }],
    fetcher,
  );

  useEffect(() => {
    if (searchResults && searchResults._links?.next?.href) {
      preload([{ endpoint: "get_next_recipes", param: { nextUrl: searchResults?._links?.next?.href } }], fetcher);
    }
  }, [searchResults]);

  const errorMessage = error ? "An error has occured. Please try again later" : null;

  if (action && searchTerm) {
    return (
      <>
        <Flex
          direction="column"
          w="100%"
          align="center"
          justify={searchResults && Number(searchResults.count) > 0 ? "flex-start" : "center"}
          gap={5}
        >
          <SearchHeader searchTerm={searchTerm} searchResults={searchResults} />

          <Flex direction="column" align="center" gap={8}>
            {searchResults && (
              <SearchResults searchResults={searchResults} excludeScrapable={excludeScrapable} goToNext={goToNext} />
            )}

            {errorMessage && !searchResults && (
              <Text fontFamily="mono" textColor="red.500">
                {errorMessage}
              </Text>
            )}
            {!searchResults && !errorMessage && (
              <Spinner message="Removing unscrapable results, may take ~20-30 seconds" />
            )}
          </Flex>
        </Flex>
      </>
    );
  }
};

export default Search;
