import {
  Accordion,
  UnorderedList,
  AccordionItem,
  AccordionButton,
  Box,
  AccordionPanel,
  AccordionIcon,
  ListItem,
} from "@chakra-ui/react";

interface AccordianProps {
  title: string;
  content: string[] | null;
}
const RecipeAccordian = ({ title, content }: AccordianProps) => {
  return (
    <Accordion w="100%" defaultIndex={[0]} allowToggle>
      <AccordionItem>
        <h2>
          <AccordionButton bg="lightgray" _hover={{ bg: "darkgray" }}>
            <Box as="span" flex="1" textAlign="left" fontFamily="mono">
              {title}
            </Box>
            <AccordionIcon />
          </AccordionButton>
        </h2>
        <AccordionPanel pb={4}>
          <UnorderedList>
            {content &&
              content.length > 0 &&
              content.map((item) => {
                if (item !== "") {
                  return (
                    <ListItem fontFamily="mono" key={item}>
                      {item}
                    </ListItem>
                  );
                }
              })}
            {!content ||
              (content.length === 0 && (
                <ListItem fontStyle="italic" fontFamily="mono">
                  Not available
                </ListItem>
              ))}
          </UnorderedList>
        </AccordionPanel>
      </AccordionItem>
    </Accordion>
  );
};

export default RecipeAccordian;
