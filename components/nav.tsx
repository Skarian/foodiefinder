import { ArrowBackIcon, SearchIcon, StarIcon } from "@chakra-ui/icons";
import { Button, Flex, Spacer, Text, useBreakpointValue } from "@chakra-ui/react";
import { useRouter } from "next/router";
import { FC, useEffect, useState } from "react";
import { getVersion } from "@tauri-apps/api/app";

async function fetchVersion() {
  try {
    const appVersion = await getVersion();
    return appVersion;
  } catch (error) {
    throw new Error("failed to fetch version number");
  }
}

const Nav: FC = () => {
  const [isRoot, setIsRoot] = useState(false);
  const router = useRouter();
  const [appVersion, setAppVersion] = useState("");

  useEffect(() => {
    fetchVersion()
      .then((version) => setAppVersion(version))
      .catch((error) => console.error(error));
  }, []);

  useEffect(() => {
    setIsRoot(router.pathname === "/");
  }, [router.pathname]);

  const goBack = () => {
    router.back();
  };

  const handleRedirect = (route: string) => {
    router.push(route as string);
  };

  const isSmallScreen = useBreakpointValue({ sm: false, base: true });

  return (
    <Flex direction="row" w="100%" justify="center">
      <Flex alignItems="center">
        <Text fontSize="md" color="white" fontWeight="bold" fontFamily="mono" noOfLines={1} userSelect="none">
          {`v${appVersion}`}
        </Text>
      </Flex>
      {!isSmallScreen && (
        <>
          <Spacer />
          <Flex gap="2" alignItems="center">
            <Button
              fontFamily="mono"
              size="xs"
              leftIcon={<ArrowBackIcon />}
              onClick={goBack}
              visibility={isRoot ? "hidden" : "visible"}
            >
              Back
            </Button>
            <Button fontFamily="mono" size="xs" leftIcon={<SearchIcon />} onClick={() => handleRedirect("/")}>
              New Search
            </Button>
            <Button fontFamily="mono" size="xs" leftIcon={<StarIcon />} onClick={() => handleRedirect("/favorites")}>
              Favorites
            </Button>
          </Flex>
        </>
      )}
    </Flex>
  );
};

export default Nav;
