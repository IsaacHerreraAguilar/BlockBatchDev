"use client" 

import React from "react";
import { PlusCircle, Users, Calendar } from "lucide-react";

const actions = [
  { icon: <PlusCircle size={18} />, label: "Create New Batch" },
  { icon: <Users size={18} />, label: "Manage Recipients" },
  { icon: <Calendar size={18} />, label: "Schedule Payments" },
];

const QuickActions = () => {
  return (
    <div className="bg-white p-4 rounded-lg shadow">
      <h2 className="text-lg font-medium mb-2">Quick Actions</h2>
      <ul className="space-y-3">
        {actions.map((action, i) => (
          <li key={i} className="flex items-center justify-between p-3 border rounded hover:bg-gray-50 cursor-pointer">
            <div className="flex items-center space-x-2">
              {action.icon}
              <span className="text-sm font-medium">{action.label}</span>
            </div>
            <span className="text-gray-400">→</span>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default QuickActions;