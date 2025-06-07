"use client";

import React, { useState } from 'react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import Image from 'next/image';
import { 
  Home, 
  FileText, 
  Users, 
  Settings, 
  Menu,
  X
} from 'lucide-react';
import logo from '/public/logo.png';

interface SidebarProps {
  children: React.ReactNode;
}

const menuItems = [
  {
    label: 'Overview',
    href: '/Dashboard',
    icon: Home
  },
  {
    label: 'Transactions',
    href: '/Transactions',
    icon: FileText
  },
  {
    label: 'Settings',
    href: '/SettingsPage',
    icon: Settings
  }
];

export default function Sidebar({ children }: SidebarProps) {
  const [isCollapsed, setIsCollapsed] = useState(false);
  const [isMobileOpen, setIsMobileOpen] = useState(false);
  const pathname = usePathname();

  const toggleSidebar = () => {
    setIsCollapsed(!isCollapsed);
  };

  const toggleMobileMenu = () => {
    setIsMobileOpen(!isMobileOpen);
  };

  return (
    <div className="flex min-h-screen bg-gray-50">
      {/* Desktop Sidebar */}
      <aside 
        className={`hidden md:flex flex-col bg-white border-r border-gray-200 transition-all duration-300 ease-in-out ${
          isCollapsed ? 'w-16' : 'w-64'
        }`}
      >
        {/* Header with Logo Only */}
        <div className="flex items-center p-4">
          {!isCollapsed && (
            <div className="flex items-center">
              <Image src={logo} width={32} height={32} alt="BlockBatch" className="mr-2" />
            </div>
          )}
          {isCollapsed && (
            <Image src={logo} width={32} height={32} alt="BlockBatch" className="mx-auto" />
          )}
        </div>

        {/* Dashboard Label */}
        {!isCollapsed && (
          <div className="px-4 py-3 border-t border-gray-200">
            <h2 className="text-sm font-medium text-gray-500 uppercase tracking-wider">Dashboard</h2>
          </div>
        )}

        {/* Navigation Menu */}
        <nav className="flex-1 px-3 py-2">
          <ul className="space-y-1">
            {menuItems.map((item) => {
              const IconComponent = item.icon;
              const isActive = pathname === item.href;
              
              return (
                <li key={item.href}>
                  <Link
                    href={item.href}
                    className={`flex items-center px-3 py-2 rounded-md text-sm font-medium transition-colors group ${
                      isActive
                        ? 'bg-gray-100 text-gray-900'
                        : 'text-gray-700 hover:bg-gray-50 hover:text-gray-900'
                    }`}
                    title={isCollapsed ? item.label : undefined}
                  >
                    <IconComponent 
                      size={20} 
                      className={`${isCollapsed ? 'mx-auto' : 'mr-3'} flex-shrink-0`}
                    />
                    {!isCollapsed && (
                      <span className="truncate">{item.label}</span>
                    )}
                  </Link>
                </li>
              );
            })}
          </ul>
        </nav>

        {/* User Info Footer */}
        {!isCollapsed && (
          <div className="border-t border-gray-200 p-4">
            <div className="flex items-center">
              <div className="w-8 h-8 bg-gray-300 rounded-full flex items-center justify-center text-sm font-medium text-gray-700">
                JD
              </div>
              <div className="ml-3 overflow-hidden">
                <p className="text-sm font-medium text-gray-900 truncate">John Doe</p>
                <p className="text-xs text-gray-500 truncate">john.doe@example.com</p>
              </div>
            </div>
          </div>
        )}
      </aside>

      {/* Mobile Sidebar Overlay */}
      {isMobileOpen && (
        <div className="fixed inset-0 bg-black bg-opacity-25 transition-opacity duration-300 ease-in-out z-30 md:hidden" onClick={toggleMobileMenu} />
      )}

      {/* Mobile Sidebar */}
      <aside className={`fixed left-0 top-0 bottom-0 w-64 bg-white border-r border-gray-200 flex flex-col transform transition-transform duration-300 ease-in-out z-40 md:hidden ${
        isMobileOpen ? 'translate-x-0' : '-translate-x-full'
      }`}>
        {/* Mobile Header */}
        <div className="flex items-center justify-between p-4 border-b border-gray-200">
          <div className="flex items-center">
            <Image src={logo} width={32} height={32} alt="BlockBatch" className="mr-2" />
            <span className="text-lg font-semibold text-gray-800">BlockBatch</span>
          </div>
          <button
            onClick={toggleMobileMenu}
            className="p-1 rounded-md hover:bg-gray-100 transition-colors"
            aria-label="Close sidebar"
          >
            <X size={20} className="text-gray-600" />
          </button>
        </div>

        {/* Mobile Dashboard Label */}
        <div className="px-4 py-3">
          <h2 className="text-sm font-medium text-gray-500 uppercase tracking-wider">Dashboard</h2>
        </div>

        {/* Mobile Navigation Menu */}
        <nav className="flex-1 px-3 py-2">
          <ul className="space-y-1">
            {menuItems.map((item) => {
              const IconComponent = item.icon;
              const isActive = pathname === item.href;
              
              return (
                <li key={item.href}>
                  <Link
                    href={item.href}
                    onClick={toggleMobileMenu}
                    className={`flex items-center px-3 py-2 rounded-md text-sm font-medium transition-colors ${
                      isActive
                        ? 'bg-gray-100 text-gray-900'
                        : 'text-gray-700 hover:bg-gray-50 hover:text-gray-900'
                    }`}
                  >
                    <IconComponent size={20} className="mr-3 flex-shrink-0" />
                    <span className="truncate">{item.label}</span>
                  </Link>
                </li>
              );
            })}
          </ul>
        </nav>

        {/* Mobile User Info Footer */}
        <div className="border-t border-gray-200 p-4">
          <div className="flex items-center">
            <div className="w-8 h-8 bg-gray-300 rounded-full flex items-center justify-center text-sm font-medium text-gray-700">
              JD
            </div>
            <div className="ml-3 overflow-hidden">
              <p className="text-sm font-medium text-gray-900 truncate">John Doe</p>
              <p className="text-xs text-gray-500 truncate">john.doe@example.com</p>
            </div>
          </div>
        </div>
      </aside>

      {/* Main Content */}
      <main className="flex-1 flex flex-col overflow-hidden">
        {/* Desktop Toggle Button */}
        <div className="hidden md:flex items-center justify-start p-4 bg-white border-b border-gray-200">
          <button
            onClick={toggleSidebar}
            className="p-2 rounded-md hover:bg-gray-100 transition-colors"
            aria-label={isCollapsed ? "Expand sidebar" : "Collapse sidebar"}
            aria-expanded={!isCollapsed}
          >
            <Image src='/icons/menu-toggle.svg' width={20} height={20} alt="Menu" className="text-gray-600" />
          </button>
        </div>

        {/* Mobile Toggle Button */}
        <div className="md:hidden bg-white border-b border-gray-200 px-4 py-3 flex items-center justify-start">
          <button
            onClick={toggleMobileMenu}
            className="p-2 rounded-md hover:bg-gray-100 transition-colors mr-2"
            aria-label="Open sidebar"
          >
            <Image src='/icons/menu-toggle.svg' width={20} height={20} alt="Menu" className="text-gray-600" />
          </button>
        </div>

        {/* Page Content */}
        <div className="flex-1 overflow-auto">
          {children}
        </div>
      </main>
    </div>
  );
} 