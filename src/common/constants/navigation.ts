import { IconType } from "react-icons";
import { FiHome, FiTrendingUp, FiCompass, FiStar } from "react-icons/fi";
import { PiWalletBold, PiUser } from "react-icons/pi";

interface LinkItemProps {
  name: string;
  to?: string;
  icon: IconType;
}

export const SidebarLinkItems: Array<LinkItemProps> = [
  { name: 'Welcome', to: '/', icon: FiHome },
  { name: 'Wallet', to: '/wallet', icon: PiWalletBold },
  { name: 'Identity', to: '/identity', icon: PiUser },
  // { name: 'Tauri', to: '/tauri', icon: FiCompass },
  // { name: 'Zustand', to: "/zustand", icon: GiMatterStates },
];
