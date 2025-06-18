"use client";

import { Button } from "@/components/ui/button";
import { Database, Menu,  } from "lucide-react";
import { motion } from "framer-motion";
import {Link} from 'react-router-dom';
import ConnectWallet from './ConnectWallet';


export default function Navbar() {
  return (
    <motion.nav
      initial={{ y: -100 }}
      animate={{ y: 0 }}
      className="flex items-center justify-between px-6 py-4 backdrop-blur-sm border-b border-white/10"
    >
      <Link href="/" className="flex items-center space-x-2">
        <Database className="w-8 h-8 text-purple-500" />
        <span className="font-medium text-xl text-white">I-migrate</span>
      </Link>

      <div className="hidden md:flex items-center space-x-8">
        <NavLink href="/about" >About</NavLink>
        <NavLink href="/dashboard">Dashboard</NavLink>
        <NavLink href="/prices">Pricing</NavLink>
      </div>

      <div className=" md:flex items-center space-x-4">
        <ConnectWallet/>
      </div>

    </motion.nav>
  );
}

function NavLink({ href, children }) {
  return (
    <Link to={href} className="text-gray-300 hover:text-white transition-colors relative group">
      {children}
      <span className="absolute -bottom-1 left-0 w-0 h-0.5 bg-purple-500 transition-all group-hover:w-full" />
    </Link>
  );
}
