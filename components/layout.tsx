import { Box, Button, ButtonGroup, Flex } from "@chakra-ui/react";
import { FC, ReactNode } from "react";
import Nav from "@/components/nav";
import Footer from "@/components/footer";

interface LayoutProps {
  children: ReactNode;
}

const Layout: FC<LayoutProps> = ({ children }) => {
  return (
    <>
      <Box h="100vh" w="100vw" overflow="hidden">
        <Flex direction="column" height="100vh">
          {/* Navbar */}
          <Flex bg="twitter.500" p={2}>
            <Nav />
          </Flex>
          {/* Page Content */}
          <Flex flex={1} p={2} overflowY="scroll">
            {children}
          </Flex>
          {/* Footer */}
          <Flex bg="twitter.500" p={2}>
            <Footer />
          </Flex>
        </Flex>
      </Box>
    </>
  );
};

export default Layout;
