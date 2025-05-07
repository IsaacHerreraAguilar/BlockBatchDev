"use client";

import React from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer
} from "recharts";

const data = [
  { name: "Jan", value: 45000 },
  { name: "Feb", value: 52000 },
  { name: "Mar", value: 48000 },
  { name: "Apr", value: 60000 },
  { name: "May", value: 68000 },
  { name: "Jun", value: 75000 },
  { name: "Jul", value: 88000 },
  { name: "Aug", value: 97000 },
  { name: "Sep", value: 110000 },
];

const TransactionOverview = () => {
  return (
    <div className="bg-white w-full max-w-[771.22px] p-4 rounded-lg shadow col-span-2">
      <h2 className="text-lg font-medium mb-2">Transaction Overview</h2>
      <div className="h-64">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={data}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="name" />
            <YAxis />
            <Tooltip />
            <Line type="monotone" dataKey="value" stroke="#4F46E5" strokeWidth={2} />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
};

export default TransactionOverview;
