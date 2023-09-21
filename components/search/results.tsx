import { Wrap, Button, Text } from "@chakra-ui/react";
import Card from "@/components/card";
import { ArrowRightIcon } from "@chakra-ui/icons";
import type { EdamamResponse } from "@/src-tauri/bindings/EdamamResponse";
import { getUnscrapableCount, getTotalCount } from "../lib/counts";

interface Props {
  searchResults: EdamamResponse;
  goToNext: (nextUrl: string) => void;
  excludeScrapable: boolean;
}

const SearchResults: React.FC<Props> = ({ searchResults, goToNext, excludeScrapable }) => {
  return (
    <>
      {Number(searchResults.count) === 0 || getTotalCount(searchResults) === getUnscrapableCount(searchResults) ? (
        <Text fontFamily="mono" textColor="red.500">
          There are <strong>0</strong> valid results sorry{" "}
        </Text>
      ) : (
        <>
          <Wrap spacing="20px" justify="center">
            {searchResults.hits.map((item, index) => {
              if (excludeScrapable && item.isScrapable && item.isValid) {
                return <Card key={index} data={item} />;
              } else if (!excludeScrapable) {
                return <Card key={index} data={item} />;
              }
            })}
          </Wrap>
          {Number(searchResults.count) > Number(searchResults.to) && (
            <Button
              fontFamily="mono"
              size="md"
              rightIcon={<ArrowRightIcon />}
              onClick={() => {
                if (searchResults._links && searchResults._links.next) {
                  goToNext(searchResults._links.next.href);
                }
              }}
              mb={8}
            >
              Next Page
            </Button>
          )}
        </>
      )}
    </>
  );
};

export default SearchResults;
