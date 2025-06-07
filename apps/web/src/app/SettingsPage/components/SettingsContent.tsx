'use client';

import { FC, useState } from "react";
import { Eye, EyeOff, Copy, RefreshCw, Trash2, ArrowRight } from "lucide-react";

interface ApiKey {
  name: string;
  key: string;
  created: string;
  lastUsed: string;
  permissions: string[];
}

const SettingsContent: FC = () => {
  // Mock data for API keys
  const [apiKeys, setApiKeys] = useState<ApiKey[]>([
    {
      name: "Production API Key",
      key: "pk_4****************7gQ",
      created: "2023-05-16",
      lastUsed: "2023-05-24",
      permissions: ["Read", "Write"]
    },
    {
      name: "Development API Key",
      key: "sk_8****************3dF",
      created: "2023-03-07",
      lastUsed: "2023-04-22",
      permissions: ["Read"]
    }
  ]);

  // State for key visibility
  const [visibleKeys, setVisibleKeys] = useState<Record<string, boolean>>({});

  // Toggle key visibility
  const toggleKeyVisibility = (keyName: string) => {
    setVisibleKeys((prev) => ({
      ...prev,
      [keyName]: !prev[keyName]
    }));
  };

  // Copy key to clipboard
  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text)
      .then(() => {
        alert("API Key copied to clipboard!");
      })
      .catch((err) => {
        console.error('Failed to copy text: ', err);
      });
  };

  // Delete API key (mock function)
  const deleteApiKey = (keyName: string) => {
    if (confirm(`Are you sure you want to delete the API key "${keyName}"?`)) {
      setApiKeys(apiKeys.filter(k => k.name !== keyName));
    }
  };

  // Rotate API key (mock function)
  const rotateApiKey = (keyName: string) => {
    if (confirm(`Are you sure you want to rotate the API key "${keyName}"? This will generate a new key and invalidate the existing one.`)) {
      // In a real application, this would call an API to generate a new key
      setApiKeys(apiKeys.map(k => {
        if (k.name === keyName) {
          return {
            ...k,
            key: `${keyName.substring(0, 2).toLowerCase()}_${'abcdef'}****************${'xyz'}`,
            created: new Date().toISOString().split('T')[0]
          };
        }
        return k;
      }));
    }
  };

  // Add new API key (mock function)
  const addNewApiKey = () => {
    const newKeyName = prompt("Enter a name for your new API key:");
    if (newKeyName) {
      const newKey: ApiKey = {
        name: newKeyName,
        key: `${newKeyName.substring(0, 2).toLowerCase()}_${'newkey'}****************${'123'}`,
        created: new Date().toISOString().split('T')[0],
        lastUsed: "Never",
        permissions: ["Read"]  // Default permission
      };
      setApiKeys([...apiKeys, newKey]);
    }
  };

  return (
    <div className="py-6">
      <div className="mb-6">
        <h2 className="text-lg font-medium mb-2">API Keys</h2>
        <p className="text-gray-600 text-sm">Manage API keys for integrating BatchPay with your systems.</p>
      </div>
      
      {/* API Keys Table */}
      <div className="mt-6 bg-white shadow overflow-hidden rounded-lg">
        <div className="px-4 py-5 sm:px-6 flex justify-between items-center">
          <h3 className="text-base font-medium">Your API Keys</h3>
          <button 
            onClick={addNewApiKey}
            className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-black hover:bg-gray-800 focus:outline-none"
          >
            + Create API Key
          </button>
        </div>
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200">
            <thead>
              <tr>
                <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Name
                </th>
                <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  API Key
                </th>
                <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Created
                </th>
                <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Last Used
                </th>
                <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Permissions
                </th>
                <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {apiKeys.map((apiKey) => (
                <tr key={apiKey.name} className="hover:bg-gray-50">
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                    {apiKey.name}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 flex items-center space-x-2">
                    <span className="font-mono">
                      {visibleKeys[apiKey.name] ? apiKey.key.replace('****************', 'FULL-KEY-REVEALED-HERE') : apiKey.key}
                    </span>
                    <button 
                      onClick={() => toggleKeyVisibility(apiKey.name)}
                      className="text-gray-400 hover:text-gray-500"
                      aria-label={visibleKeys[apiKey.name] ? "Hide API Key" : "Show API Key"}
                    >
                      {visibleKeys[apiKey.name] ? <EyeOff size={16} /> : <Eye size={16} />}
                    </button>
                    <button 
                      onClick={() => copyToClipboard(apiKey.key)}
                      className="text-gray-400 hover:text-gray-500"
                      aria-label="Copy API Key"
                    >
                      <Copy size={16} />
                    </button>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {apiKey.created}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {apiKey.lastUsed}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    <div className="flex flex-wrap gap-2">
                      {apiKey.permissions.map((permission) => (
                        <span key={`${apiKey.name}-${permission}`} className={`px-2 py-1 text-xs rounded-full ${
                          permission === 'Read' ? 'bg-blue-100 text-blue-800' : 
                          permission === 'Write' ? 'bg-green-100 text-green-800' : 
                          'bg-gray-100 text-gray-800'
                        }`}>
                          {permission}
                        </span>
                      ))}
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 space-x-2">
                    <button 
                      onClick={() => rotateApiKey(apiKey.name)}
                      className="text-gray-400 hover:text-gray-500"
                      aria-label="Rotate API Key"
                    >
                      <RefreshCw size={16} />
                    </button>
                    <button 
                      onClick={() => deleteApiKey(apiKey.name)}
                      className="text-red-400 hover:text-red-500"
                      aria-label="Delete API Key"
                    >
                      <Trash2 size={16} />
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
      
      {/* API Documentation Section */}
      <div className="mt-10 bg-white shadow overflow-hidden rounded-lg">
        <div className="px-4 py-5 sm:p-6">
          <h3 className="text-base font-medium">API Documentation</h3>
          <p className="mt-2 text-sm text-gray-500">
            Learn how to integrate BatchPay with your systems using our comprehensive API documentation.
          </p>
          <div className="mt-4">
            <a 
              href="#" 
              className="inline-flex items-center text-sm font-medium text-black hover:text-gray-700"
            >
              View API Documentation
              <ArrowRight className="ml-1" size={16} />
            </a>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SettingsContent;
