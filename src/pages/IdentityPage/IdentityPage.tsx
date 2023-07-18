import { Text } from '@chakra-ui/react';
import React from 'react'
import { IoLogInOutline } from 'react-icons/io5'
import { useWalletStore } from '../../stores/useWalletStore';  // replace with your actual store path
import { BoxAction } from '@/pages/IdentityPage/components';
import { VerifyUser } from '@/pages/IdentityPage/components/index';
interface CustomResponse {
  message: string
}

const IdentityPage: React.FC = () => {
  const { eraWallet, setWallet } = useWalletStore((state) => ({
    eraWallet: state.eraWallet,
    setWallet: state.setWallet
  }));

  return (
    <BoxAction title="Identity" icon={IoLogInOutline}>
      {eraWallet ? <VerifyUser /> : <Text>You must create a wallet first</Text>}
    </BoxAction>
  )
}

export default IdentityPage;
