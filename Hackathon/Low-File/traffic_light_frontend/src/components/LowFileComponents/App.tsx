import React, { useState } from 'react';
import { ChakraProvider, Box, Button, FormControl, FormLabel, Input, Text, Center, Flex } from '@chakra-ui/react';
import { useSailsCalls } from '@/app/hooks';
import { useAccount } from '@gear-js/react-hooks';
import { web3FromSource } from '@polkadot/extension-dapp';

function App() {
  const [showProfile, setShowProfile] = useState(false); // Estado para manejar la pantalla
  const [userData, setUserData] = useState({ // Estado para guardar los datos del formulario
    nombre: '',
    apellidos: '',
    fechaNacimiento: '',
    correo: '',
    password: ''
  });

  const handleContinue = () => {
    setShowProfile(true); // Cambiar a la pantalla de perfil
  };

  return (
    <ChakraProvider>
      <Center h="100vh" bg="gray.100">
        {/* Si showProfile es verdadero, muestra el perfil, de lo contrario, muestra el registro */}
        {showProfile ? <ProfileScreen userData={userData} /> : <RegisterScreen userData={userData} setUserData={setUserData} onContinue={handleContinue} />}
      </Center>
    </ChakraProvider>
  );
}

function RegisterScreen({ onContinue, userData, setUserData }) {
  const{account} =useAccount()
  const sails = useSailsCalls()
  
  const [nombre, setNombre] = useState('');
  const [apellidos, setApellidos] = useState('');
  const [fechaNacimiento, setFechaNacimiento] = useState('');
  const [correo, setCorreo] = useState('');
  const [password, setPassword] = useState('');

  const handleChange = (e) => {
    const { name, value } = e.target;
    setUserData({ ...userData, [name]: value });
  };

const next = async()=>{
  if(!sails){
    console.log("No esta inicializado sails")
    return
  }
  if(!account){
    console.log("No esta inicializado account")
    return
  }
  console.log(nombre)
  console.log(password)
  console.log(apellidos)
  console.log(correo)

  const{signer}=await web3FromSource(account.meta.source)

  const response = await sails.command(
    "LowFileService/SetUserData",
    {
      userAddress:account.decodedAddress,
      signer
    },
    {
      callArguments:[
        nombre,
        0,
        "",
        "",
        "",
        [],
        "",
        [],
        correo,
        apellidos, 
      ]
    }
  )

}

  return (
      <Flex w="100%" maxW="800px" p={4} bg="white" boxShadow="lg" borderRadius="md" direction="row" justify="space-between" align="center">
        {/* Formulario de registro */}
        <Box w="50%" p={4}>
          <Text fontSize="2xl" fontWeight="bold" textAlign="left" mb={2}>
            Regístrate para tu cuenta de Low File
          </Text>
          <Text fontSize="md" color="gray.500" mb={4}>
            Tu huella digital aquí y ahora
          </Text>
          <FormControl mb={4}>
            <FormLabel>Nombre</FormLabel>
            <Input
              name="nombre"
              type="text"
              value={nombre}
              onChange={(e)=>setNombre(e.target.value)}
            />
          </FormControl>
          <FormControl mb={4}>
            {/* Cambiar Apellidos a Usario */}
            <FormLabel>Usario</FormLabel>
            <Input
              name="apellidos"
              type="text"
              value={apellidos}
              onChange={(e)=>setApellidos(e.target.value)}
            />
          </FormControl>
          <FormControl mb={4}>
            <FormLabel>Fecha de Nacimiento</FormLabel>
            <Input
              name="fechaNacimiento"
              type="date"
              value={userData.fechaNacimiento}
              onChange={handleChange}
            />
          </FormControl>
          <FormControl mb={4}>
            <FormLabel>Correo Electrónico</FormLabel>
            <Input
              name="correo"
              type="email"
              value={correo}
              onChange={(e)=>setCorreo(e.target.value)}
            />
          </FormControl>
          <FormControl mb={6}>
            <FormLabel>Contraseña</FormLabel>
            <Input
              name="password"
              type="password"
              value={password}
              onChange={(e)=>setPassword(e.target.value)}
            />
          </FormControl>
          <Button colorScheme="blue" width="100%" onClick={async()=>{
            await next()
            onContinue()
          }}>
            Continuar
          </Button>
        </Box>

        {/* Información adicional */}
        <Box w="50%" p={4} bg="gray.50" borderRadius="md" textAlign="left">
          <Text fontSize="xl" fontWeight="bold" mb={2}>
            Crea tu cuenta <span style={{ color: 'blue' }}>gratis</span>
          </Text>
          <Text mb={4}>Con una cuenta gratuita, descubre cómo cuidar tus datos y mantener tu identidad digital bajo control.</Text>
          <ul style={{ marginLeft: '20px', listStyle: 'disc' }}>
            <li>Detecta actividad reciente en tus perfiles y cuentas vinculadas.</li>
            <li>Accede a un resumen de tu información desde un solo lugar.</li>
          </ul>
        </Box>
      </Flex>
  );
}

function ProfileScreen({ userData }) {
  return (
    <Box w="100%" maxW="1500px" p={4} bg="white" boxShadow="lg" borderRadius="md" textAlign="center">
      <Text fontSize="4xl" fontWeight="bold">
        {userData.nombre} {userData.apellidos}
      </Text>
      <Text mb={2}>@{userData.nombre.toLowerCase().split(' ').join('_')}</Text>
      <Text>Nombre Completo: {userData.nombre} {userData.apellidos}</Text>
      <Text>Fecha de Nacimiento: {new Date(userData.fechaNacimiento).toLocaleDateString()}</Text>
      <Text>Correo Electrónico: {userData.correo}</Text>
    </Box>
  );
}

export { App };
