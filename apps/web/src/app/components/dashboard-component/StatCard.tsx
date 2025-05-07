import React from 'react';

const StatCard = ({ title, value, change }) => {
    return (
      <div className="bg-white p-4 rounded-lg shadow">
        <h3 className="text-sm text-gray-500 mb-1">{title}</h3>
        <p className="text-xl font-semibold text-gray-800">{value}</p>
        <p className="text-xs text-green-500 mt-1">{change}</p>
      </div>
    );
  };
  
  export default StatCard;