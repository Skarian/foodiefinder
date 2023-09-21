import { convertFileSrc } from "@tauri-apps/api/tauri";

export const getRightImage = (url: string): string => {
  if (url.startsWith("/")) {
    return convertFileSrc(url);
  } else {
    return url;
  }
};
