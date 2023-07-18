import { Box, Text } from '@chakra-ui/react';
import { Heading, Button, Link } from '@chakra-ui/react';
import { invoke } from '@tauri-apps/api/tauri'
import { useWalletStore } from '../../../stores/useWalletStore';  // replace with your actual store path

import React, { useState } from 'react';

interface CustomResponse {
  message: string
}

export const VerifyUser = () => {
  const { eraWallet } = useWalletStore((state) => ({
    eraWallet: state.eraWallet
  }));
  const [kycInfo, setKycInfo] = useState<any | null>(null)
  const [userData, setUser] = useState<any | null>(null)
  const [userProof, setUserProof] = useState<any | null>(null)


  const verifyKYCHandler = async (event) => {
    event.preventDefault();

    if (eraWallet === null) return

    const res: CustomResponse = await invoke('create_veriff_session');
    if (res !== undefined) {
      const parsed = JSON.parse(JSON.parse(res.message))

      setKycInfo(parsed);
    } else {
      console.error(res)
    }
  }

  const updateStatus = async () => {
    verifyKYCHandler(null)
  }

  const generateProof = async () => {
    const sessionToken = kycInfo?.verification?.sessionToken;
    const sessionid = kycInfo?.verification?.id;
    if (eraWallet === null || kycInfo?.verification === null) return
    const res: CustomResponse = await invoke('generate_proof', { wallet: eraWallet, sessiontoken: sessionToken, sessionid: sessionid });

    if (res !== undefined) {
      const parsed = JSON.parse(JSON.parse(res.message))
      setUserProof(parsed);
    } else {
      console.error(res)
    }
  }

  const Items = () => {
    if (kycInfo !== null) {
      return (
        <>
          <Text>Status: {kycInfo?.verification?.status ?? "-"}</Text>
          <Link href={kycInfo?.verification?.url} isExternal>
            <Button type="submit">
              Complete verification
            </Button>
          </Link>
          <Button onClick={updateStatus} mt={5}>
            Update status
          </Button>
          <Button onClick={generateProof} mt={5}>
            Create Private Proof
          </Button>
        </>
      )
    } else {
      return (
        <div>
          <Text>Step 1</Text>
          <Text>Complete verification</Text>
          <Box display="flex" justifyContent="flex-end">
            <Button type="submit" onClick={verifyKYCHandler}>
              Verify
            </Button>
          </Box>
        </div>
      )
    }

  }

  return (
    <Box maxW="400px" mx="auto" p={4}>
      <Text>{JSON.stringify(kycInfo)}</Text>
      {/* <Text>Address: {JSON.stringify(eraWallet)}</Text> */}
      <Heading as='h5' mb={4}>Verify and mint</Heading>
      <Items />
    </Box>
  );
};