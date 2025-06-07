"use client";
import React, { useState } from "react";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow
} from "@/components/ui/table";
import { RecipientsProps } from "@/lib/interface";
import { Copy } from "lucide-react";
import { ExternalLink } from "lucide-react";
import { User } from "lucide-react";
import { Toaster } from "@/components/ui/sonner";
import { toast } from "sonner";

const RecipientTable = ({ recipients }: { recipients: RecipientsProps[] }) => {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const [copied, setCopied] = useState(false);

  const handleCopy = async (text: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopied(true);
      toast("Copied to clipboard");

      setTimeout(() => {
        setCopied(false);
      }, 2000);
    } catch (error) {
      console.error("Failed to copy", error);
    }
  };

  return (
    <div className="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead className="text-[#71717A] font-medium">Name</TableHead>
            <TableHead className="text-[#71717A] font-medium">Email</TableHead>
            <TableHead className="text-[#71717A] font-medium">
              Wallet Address
            </TableHead>
            <TableHead className="text-[#71717A] font-medium">Status</TableHead>
            <TableHead className="text-[#71717A] font-medium">
              Total Paid
            </TableHead>
            <TableHead className="text-[#71717A] font-medium">
              Actions
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {recipients.map((recipient) => (
            <TableRow key={recipient.id}>
              <TableCell className="">
                <div className="flex items-center">
                  <div className="w-8 font-medium h-8 rounded-full bg-gray-200 flex items-center justify-center mr-2">
                    {recipient.id}
                  </div>
                  <div>
                    <h6 className="font-medium">{recipient.name}</h6>
                    <p className="text-[#71717A] text-[12px]">
                      {recipient.type}
                    </p>
                  </div>
                </div>
              </TableCell>
              <TableCell>{recipient.email}</TableCell>
              <TableCell className="flex items-center justify-start  gap-2">
                <span> {recipient.walletAddress.slice(0, 14) + "..."}</span>
                <span className="flex items-center gap-1">
                  <button className="cursor-pointer">
                    <Copy
                      className="h-4 w-4"
                      onClick={() => handleCopy(recipient.walletAddress)}
                    />
                  </button>
                  <button className="cursor-pointer">
                    <ExternalLink className="h-4 w-4" />
                  </button>
                </span>
              </TableCell>
              <TableCell>
                <span
                  className={`px-1 py-1 rounded-full text-xs font-semibold flex gap-1 items-center  ${
                    recipient.status === "Active"
                      ? "bg-[#F0FDF4] border border-[#BBF7D0] text-[#15803D]"
                      : "bg-[#F9FAFB] border border-[#E5E7EB] text-[#374151]"
                  }`}
                >
                  <User size={16} /> {recipient.status}
                </span>
              </TableCell>
              <TableCell className="font-medium">
                {recipient.totalPaid}
              </TableCell>
              <TableCell>
                <button className="text-gray-500 hover:text-gray-700">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    className="h-5 w-5"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                  >
                    <path d="M6 10a2 2 0 11-4 0 2 2 0 014 0zM12 10a2 2 0 11-4 0 2 2 0 014 0zM16 12a2 2 0 100-4 2 2 0 000 4z" />
                  </svg>
                </button>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
      <Toaster className="bottom-200 bg-green-700" />
    </div>
  );
};

export default RecipientTable;
