import type { EdamamResponse } from "@/src-tauri/bindings/EdamamResponse";

export const getUnscrapableCount = (res: EdamamResponse): number => {
  let count = 0;
  for (let i = 0; i < res.hits.length; i++) {
    if (!res.hits[i].isScrapable || !res.hits[i].isValid) {
      count += 1;
    }
  }
  return count;
};

export const getTotalCount = (res: EdamamResponse): number => {
  let count = 0;
  for (let i = 0; i < res.hits.length; i++) {
    count += 1;
  }
  return count;
};
