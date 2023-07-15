import { Box, ButtonProps, Text } from '@chakra-ui/react';
import { Heading, FormControl, FormLabel, Input, Button } from '@chakra-ui/react';
import { invoke } from '@tauri-apps/api/tauri'

import React from 'react';
import { IconType } from 'react-icons';

type ButtonWithIconProps = ButtonProps & {
  icon: IconType;
  label: string;
}

interface CustomResponse {
  message: string
}

export const CreateWallet = (props: ButtonWithIconProps) => {

  const createWalletHandler = async (event) => {
    event.preventDefault();

    const res: CustomResponse = await invoke('create_zksync_wallet', { ethereumpk: '0x7726827caac94a7f9e1b160f7ea819f172f7b6f9d2a97f992c38edeab82d4110' });
    if (res !== undefined) {
      console.log("🚀 ~ file: CreateWallet.tsx:25 ~ createWalletHandler ~ res.message:", res.message)
      // setRustMessage(res.message)
    } else {
      console.error(res)
    }
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
          <Input type="password" placeholder="0x..." />
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