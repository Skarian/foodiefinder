import { invoke } from "@/components/lib/tauri";
import useSWR from "swr";

type EmptyParam = Record<string, never>;

type FetcherParam =
  | { endpoint: "get_all_recipes"; param: EmptyParam }
  | { endpoint: "get_recipe_by_id"; param: { id: number } }
  | { endpoint: "does_recipe_exist_by_url"; param: { url: string } };

const fetcher = async <T>([{ endpoint, param }]: FetcherParam[]): Promise<T> => {
  try {
    const value = await invoke(endpoint, param);
    return value as T;
  } catch (error) {
    console.error(error);
    throw error;
  }
};

// Custom Hook for fetching data
export const useFetchData = <T>(params: FetcherParam) => {
  const { data, error, isLoading } = useSWR<T, Error>([params], fetcher);

  return { data, error, isLoading };
};

// Example usage
//
// const { data: recipes, error } = useFetchData<RecipeDetails[]>({
//   endpoint: "get_all_recipes",
//   param: {},
// });
//
// // const { data: info, error } = useFetchData<RecipeDetails>({
//   endpoint: "get_recipe_by_id",
//   param: {id: 5},
// });
