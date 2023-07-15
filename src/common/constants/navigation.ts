import { IconType } from "react-icons";
import { FiHome, FiTrendingUp, FiCompass, FiStar } from "react-icons/fi";
import { PiWalletBold } from "react-icons/pi";
import { GiMatterStates } from "react-icons/gi";

interface LinkItemProps {
  name: string;
  to?: string;
  icon: IconType;
}

export const SidebarLinkItems: Array<LinkItemProps> = [
  { name: 'Welcome', to: '/', icon: FiHome },
  { name: 'Wallet', to: '/wallet', icon: PiWalletBold },
  { name: 'Tauri', to: '/tauri', icon: FiCompass },
  { name: 'Zustand', to: "/zustand", icon: GiMatterStates },
];
