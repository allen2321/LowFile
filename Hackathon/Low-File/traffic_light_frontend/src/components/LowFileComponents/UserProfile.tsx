import { Box, Flex, Avatar, Heading, Text, VStack, Divider } from "@chakra-ui/react";

// Definición de tipos para las propiedades del perfil de usuario
interface UserProfileProps {
  name: string;
  username: string;
  joinedDate: string;
  fullName: string;
  birthDate: string;
  birthPlace: string;
  curp: string;
  address: string;
}

const UserProfile: React.FC<UserProfileProps> = ({
  name,
  username,
  joinedDate,
  fullName,
  birthDate,
  birthPlace,
  curp,
  address,
}) => {
  return (
    <Flex
      width="100vw"
      height="100vh"
      bg="gray.50"
      justifyContent="center"
      alignItems="center"
      padding="5"
    >
      <Box
        width={{ base: "90%", md: "60%", lg: "40%" }}
        bg="white"
        boxShadow="xl"
        borderRadius="md"
        padding="6"
      >
        {/* Avatar y nombre */}
        <Flex alignItems="center" mb="6">
          <Avatar
            size="xl"
            src="https://bit.ly/dan-abramov" // Cambia por la imagen real del usuario si es necesario
            name={name}
          />
          <VStack align="flex-start" ml="4">
            <Heading size="lg">{name}</Heading>
            <Text color="gray.500">@{username}</Text>
            <Text color="gray.500">Se unió en {joinedDate}</Text>
          </VStack>
        </Flex>

        {/* Información Personal */}
        <Heading size="md" mb="4">
          Información personal
        </Heading>
        <Divider mb="4" />
        <VStack spacing="3" align="flex-start">
          <Text>
            <strong>Nombre Completo:</strong> {fullName}
          </Text>
          <Text>
            <strong>Fecha de Nacimiento:</strong> {birthDate}
          </Text>
          <Text>
            <strong>Lugar de Nacimiento:</strong> {birthPlace}
          </Text>
          <Text>
            <strong>CURP:</strong> {curp}
          </Text>
          <Text>
            <strong>Domicilio:</strong> {address}
          </Text>
        </VStack>
      </Box>
    </Flex>
  );
};

export {UserProfile};
