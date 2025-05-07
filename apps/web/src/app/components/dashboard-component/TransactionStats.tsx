"use client"

import React from "react";
import { PieChart, Pie, Cell, Legend, Tooltip } from "recharts";

const data = [
  { name: "Payroll", value: 45 },
  { name: "Vendor", value: 25 },
  { name: "Refunds", value: 15 },
  { name: "Other", value: 15 },
];

const COLORS = ["#18181B", "#18181B66", "#18181B99", "#18181BCC"];

const TransactionStats = () => {
  return (
    <div className="bg-white p-4 rounded-lg shadow">
      <h2 className="text-lg font-medium mb-2">Transaction Stats</h2>
      <div className="flex justify-center">
        <PieChart width={250} height={200}>
          <Pie
            data={data}
            cx="50%"
            cy="50%"
            innerRadius={40}
            outerRadius={60}
            fill="#8884d8"
            paddingAngle={3}
            dataKey="value"
            label
          >
            {data.map((entry, index) => (
              <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
            ))}
          </Pie>
          <Tooltip />
          <Legend verticalAlign="bottom" height={36} />
        </PieChart>
      </div>
    </div>
  );
};

export default TransactionStats;
