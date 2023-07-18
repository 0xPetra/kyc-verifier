import { Box, InputLeftAddon, InputRightElement, InputGroup, Text } from '@chakra-ui/react';
import { Heading, FormControl, FormLabel, Input, Button } from '@chakra-ui/react';
import { invoke } from '@tauri-apps/api/tauri'
import { useWalletStore } from '../../../stores/useWalletStore';  // replace with your actual store path

import React, { useState } from 'react';
import { FiCheck } from 'react-icons/fi';


interface CustomResponse {
  message: string
}

export const CreateWallet = () => {
  const { eraWallet, setWallet } = useWalletStore((state) => ({
    eraWallet: state.eraWallet,
    setWallet: state.setWallet
  }));

  const [privateKey, setPrivateKey] = useState('0xa387d6f3f28de1822eb99a333c62a4426cefff234dd32659e9d331aad7cba907');
  const [applicantid, setApplicantId] = useState<String | null>('null');
  const [value, setValue] = React.useState('')

  const handleValueChange = (event) => setValue(event.target.value)

  const handleChange = (event) => {
    setPrivateKey(event.target.value);
  }

  const createWalletHandler = async (event) => {
    event.preventDefault();

    const res: CustomResponse = await invoke('create_zksync_wallet', { ethereumpk: privateKey });
    if (res !== undefined) {
      setWallet(res.message);
    } else {
      console.error(res)
    }
  }

  const sendEth = async (event) => {
    event.preventDefault();

    if (eraWallet === null || applicantid === null) return

    const res: CustomResponse = await invoke('create_zksync_transfer', { wallet: eraWallet, amount: value });
    if (res !== undefined) {
      // TODO: Success!
    } else {
      console.error(res)
    }
  }

  if (eraWallet !== null) {
    return (
      <Box maxW="400px" mx="auto" p={4}>
        <Heading as='h5' mb={4}>Your Wallet</Heading>
        <Text>{eraWallet}</Text>

        <InputGroup>
          <InputLeftAddon children='ETH' />
          <Input
            value={value}
            onChange={handleValueChange}
            placeholder='Enter amount'
          />
          <InputRightElement>
            <FiCheck color='green.500' />
          </InputRightElement>
        </InputGroup>

        <Box display="flex" justifyContent="flex-end">
          <Button type="submit" onClick={sendEth}>
            Send Eth
          </Button>
        </Box>

      </Box>
    )
  }

  return (
    <Box maxW="400px" mx="auto" p={4}>
      <Heading as='h5' mb={4}>Create a Wallet</Heading>
      <form>
        <Text fontSize='xl' mt="5">Details</Text>
        <FormControl mb={4}>
          <FormLabel>Wallet Name</FormLabel>
          <Input focusBorderColor='lime' type="text" placeholder="Enter wallet name" />
        </FormControl>
        <FormControl mb={4}>
          <FormLabel>Private Key*</FormLabel>
          <Input
            type="password"
            placeholder="0x..."
            value={privateKey}
            onChange={handleChange}
          />
        </FormControl>

        <Text fontSize='xl' mt="5">Security Rules</Text>
        <FormControl mb={4}>
          <FormLabel>Spending Limit</FormLabel>
          <Input type="number" placeholder="0.1" />
        </FormControl>
        <FormControl mb={4}>
          <FormLabel>Time-Locked (in hours)</FormLabel>
          <Input type="number" placeholder="0" />
        </FormControl>
        <Box display="flex" justifyContent="flex-end">
          <Button type="submit" onClick={createWalletHandler}>
            Create Wallet
          </Button>
        </Box>
      </form>

      <Text>* Obligatory field</Text>
    </Box>
  );
};