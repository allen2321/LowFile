import { 
  Center, 
  HStack, 
  VStack, 
} from "@chakra-ui/react";
import { 
  App,
  UserProfile
} from "@/components";

export const Home = () => {
  return (
      <Center>
        <HStack>
          <VStack>
            <App />
          </VStack>
        </HStack>
      </Center>
    );
};