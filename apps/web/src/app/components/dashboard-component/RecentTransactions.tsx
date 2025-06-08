import React from "react";

type TransactionStatus = "Completed" | "Pending" | "Failed";

interface Transaction {
  id: string;
  date: string;
  desc: string;
  amount: string;
  recipients: number;
  status: TransactionStatus;
}

const transactions: Transaction[] = [
  { id: "TX-7829", date: "2023-03-15", desc: "March Payroll", amount: "$45,500.00", recipients: 42, status: "Completed" },
  { id: "TX-7830", date: "2023-03-16", desc: "Vendor Payments", amount: "$12,400.00", recipients: 15, status: "Completed" },
  { id: "TX-7831", date: "2023-03-17", desc: "Affiliate Commissions", amount: "$8,750.00", recipients: 23, status: "Pending" },
  { id: "TX-7832", date: "2023-03-18", desc: "Refund Batch", amount: "$2,340.00", recipients: 8, status: "Failed" },
  { id: "TX-7833", date: "2023-03-19", desc: "Contractor Payments", amount: "$18,200.00", recipients: 12, status: "Pending" },
];

const statusColor = {
  Completed: "text-green-600 bg-green-100",
  Pending: "text-yellow-600 bg-yellow-100",
  Failed: "text-red-600 bg-red-100",
};

const RecentTransactions = () => {
  return (
    <div className="bg-white w-full max-w-[771.22px] p-4 rounded-lg shadow col-span-2">
      <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-2 gap-2">
        <h2 className="text-lg font-medium">Recent Transactions</h2>
        <button className="text-sm text-gray-500 border px-3 py-1 rounded hover:bg-gray-100">Export</button>
      </div>
      <div className="overflow-auto">
        <table className="min-w-full text-sm text-left">
          <thead>
            <tr className="text-gray-500 whitespace-nowrap">
              <th className="py-2 px-2">ID</th>
              <th className="px-2">Date</th>
              <th className="px-2">Description</th>
              <th className="px-2">Amount</th>
              <th className="px-2">Recipients</th>
              <th className="px-2">Status</th>
            </tr>
          </thead>
          <tbody>
            {transactions.map((tx) => (
              <tr key={tx.id} className="border-t">
                <td className="py-2 px-2">{tx.id}</td>
                <td className="px-2">{tx.date}</td>
                <td className="px-2">{tx.desc}</td>
                <td className="px-2">{tx.amount}</td>
                <td className="px-2">{tx.recipients}</td>
                <td className="px-2">
                  <span className={`text-xs px-2 py-1 rounded ${statusColor[tx.status]}`}>
                    {tx.status}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default RecentTransactions;
