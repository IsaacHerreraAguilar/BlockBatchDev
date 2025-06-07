"use client";
import React, { useEffect, useState } from "react";
import { Search } from "lucide-react";
import { recipients } from "@/lib/mock-data";
import RecipientTable from "../components/RecipientTable";
import { Button } from "../components/ui/Button";
import { Import } from "lucide-react";
import { ArrowUpFromLineIcon } from "lucide-react";
import { Plus } from "lucide-react";
import { cn } from "@/lib/utils";

const RecipientPage = () => {
  const [tableData, setTableData] = useState(recipients);
  const [activeTab, setActiveTab] = useState("all");
  const [search, setSearch] = useState("");

  useEffect(() => {
    if (search) {
      setTableData(
        recipients.filter(
          (item) => item.email.includes(search) || item.name.includes(search)
        )
      );
    } else {
      if (activeTab === "individuals") {
        return setTableData(
          recipients.filter((item) => item.type === "Individual")
        );
      } else if (activeTab === "companies") {
        return setTableData(
          recipients.filter((item) => item.type === "Company")
        );
      } else {
        setTableData(recipients);
      }
    }
  }, [activeTab, search]);

  return (
    <div className="w-full overflow-x-auto px-4 py-4 sm:px-10 sm:py-8">
      <div className="grid gap-4 md:flex pt-6 pb-4 items-center justify-between">
        <div>
          <h1 className=" text-2xl md:text-4xl text-[#3F3F46]">Recipients</h1>
          <p className="text-[#71717A] text-[15px] md:text-[18px]">
            Manage your payment recipients
          </p>
        </div>

        <div className="flex gap-4 max-h-[1.7rem] md:max-h-[2.3rem] flex-wrap pb-24 sm:pb-2">
          <button className=" border border-[#E4E4E7] cursor-pointer text-black flex items-center p-1 px-3 gap-1 rounded-sm hover:bg-gray-100">
            <Import className="w-4 h-4" /> Import
          </button>
          <button className="border border-[#E4E4E7] cursor-pointer text-black flex items-center p-1 px-3 gap-1 rounded-sm hover:bg-gray-100">
            <ArrowUpFromLineIcon className="w-4 h-4" /> Export
          </button>
          <Button className="hover:bg-gray-800">
            <Plus /> Add Recipient
          </Button>
        </div>
      </div>

      <div className="border border-[#E4E4E7] rounded-md py-6">
        <div className="pl-4">
          {" "}
          <h2 className="text-xl sm:text-2xl text-[#09090B] font-semibold">
            Payment Recipients
          </h2>
          <p className="text-[13px] sm:text-[14px] text-[#71717A]">
            View and manage your payment recipients
          </p>
        </div>
        <div className="grid gap-4 sm:flex justify-between  items-center py-4">
          <div className="flex-wrap flex space-x-2 bg-[#F4F4F5] px-1 py-1 text-center sm:text-left">
            <button
              className={
                (cn(""),
                activeTab === "all"
                  ? "bg-white px-3 py-3 text-sm text-black font-medium"
                  : "px-3 py-3 text-sm text-[#71717A] font-medium rounded-sm cursor-pointer hover:bg-white")
              }
              onClick={() => {
                setActiveTab("all");
              }}
            >
              All Recipients
            </button>
            <button
              className={
                (cn("cursor-pointer"),
                activeTab === "individuals"
                  ? "bg-white px-3 py-3 text-sm text-black font-medium"
                  : "px-3 py-3 text-sm text-[#71717A] font-medium rounded-sm cursor-pointer hover:bg-white")
              }
              onClick={() => {
                setActiveTab("individuals");
              }}
            >
              Individuals
            </button>
            <button
              className={
                (cn("cursor-pointer"),
                activeTab === "companies"
                  ? "bg-white px-3 py-3  text-sm text-black font-medium"
                  : "px-3 py-3 text-sm text-[#71717A] font-medium rounded-sm cursor-pointer hover:bg-white")
              }
              onClick={() => {
                setActiveTab("companies");
              }}
            >
              Companies
            </button>
            <button className="px-3 py-3 cursor-pointer text-sm text-[#71717A] font-medium rounded-sm hover:bg-white">
              Groups
            </button>
          </div>

          <div className="flex gap-4 px-2">
            <div className="flex items-center gap-1  sm:px-2 border rounded-sm">
              <Search className="text-gray-600 w-5 h-5 " />

              <input
                type="text"
                placeholder="Search recipients..."
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                className="border-none w-full pl-0 focus-visible:ring-0 focus-visible:outline-0 focus-within:border-0 text-[13px] p-1 sm:text-[15px] ml-auto px-3 py-1 text-sm "
              />
            </div>
            <select className="ml-2 px-3 py-1 text-sm border rounded-sm">
              <option>All</option>
            </select>
          </div>
        </div>
        <RecipientTable recipients={tableData} />
      </div>
    </div>
  );
};

export default RecipientPage;
