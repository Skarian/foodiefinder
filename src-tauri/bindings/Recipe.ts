// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Digest } from "./Digest";
import type { Images } from "./Images";
import type { Ingredient } from "./Ingredient";
import type { TotalDaily } from "./TotalDaily";
import type { TotalNutrients } from "./TotalNutrients";

export interface Recipe { uri: string, label: string, image: string, images: Images, source: string, url: string, shareAs: string, yield: number, dietLabels: Array<string>, healthLabels: Array<string>, cautions: Array<string>, ingredientLines: Array<string>, ingredients: Array<Ingredient>, calories: number, totalWeight: number, totalTime: number, cuisineType: Array<string>, mealType: Array<string>, dishType: Array<string>, totalNutrients: TotalNutrients, totalDaily: TotalDaily, digest: Array<Digest>, }