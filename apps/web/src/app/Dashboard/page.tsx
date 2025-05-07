"use client";

import React, { FC } from "react";
import Header from "../components/dashboard-component/Header";
import StatCard from "../components/dashboard-component/StatCard";
import TransactionOverview from "../components/dashboard-component/TransactionOverview";
import TransactionStats from "../components/dashboard-component/TransactionStats";
import RecentTransactions from "../components/dashboard-component/RecentTransactions";
import QuickActions from "../components/dashboard-component/QuickActions";

const Dashboard: FC = () => {
  return (
    <main className="min-h-screen bg-gray-50 px-4 md:px-8 lg:px-12 py-6">
      <div className="max-w-[1171px] mx-auto">
        <Header />

        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 my-6">
          <StatCard title="Total Transactions" value="1,284" change="+12.5% from last month" />
          <StatCard title="Transaction Volume" value="$542,897" change="+8.2% from last month" />
          <StatCard title="Active Batches" value="6" change="2 pending confirmation" />
          <StatCard title="Average Fee" value="0.15%" change="-0.03% from last month" />
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <div className="lg:col-span-2">
            <TransactionOverview />
          </div>
          <TransactionStats />
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mt-6">
          <div className="lg:col-span-2">
            <RecentTransactions />
          </div>
          <QuickActions />
        </div>
      </div>
    </main>
  );
};

export default Dashboard;
