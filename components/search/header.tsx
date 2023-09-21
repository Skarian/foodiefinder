import React from "react";
import type { EdamamResponse } from "@/src-tauri/bindings/EdamamResponse";
import { Text } from "@chakra-ui/react";
import { getUnscrapableCount } from "@/components/lib/counts";

interface Props {
  searchTerm: string | string[];
  searchResults: EdamamResponse | null | undefined;
}

const SearchHeader: React.FC<Props> = ({ searchTerm, searchResults }) => {
  return (
    <Text fontFamily="mono" fontSize="2xl" textAlign="center">
      Results for <strong>{searchTerm}</strong>
      {searchResults && Number(searchResults.count) > 0 && (
        <Text fontFamily="mono" fontSize="md" fontStyle="italic" textColor="gray.500">
          About ~{Number(searchResults.count)} results (Showing {Number(searchResults.from)}-{Number(searchResults.to)},
          excluding {getUnscrapableCount(searchResults)})
        </Text>
      )}
    </Text>
  );
};

export default SearchHeader;
