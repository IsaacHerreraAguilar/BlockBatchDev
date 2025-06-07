import { RecipientsProps } from "./interface";

export const recipients: RecipientsProps[] = [
  {
    id: "JS",
    name: "John Smith",
    email: "john@example.com",
    type: "Individual",
    walletAddress: "ox1s2b3c4e5e6iWedr87",
    status: "Active",
    totalPaid: "$12,450.00"
  },
  {
    id: "SJ",
    name: "Sarah Johnson",
    email: "sarah@example.com",
    type: "Individual",
    walletAddress: "ox2b3c4e5e6f7g0i9s",
    status: "Active",
    totalPaid: "$8,200.00"
  },
  {
    id: "AC",
    name: "Acme Corporation",
    email: "payments@acme.com",
    type: "Company",
    walletAddress: "ox3c4d5e6f7g8nXhgyd",
    status: "Active",
    totalPaid: "$45,750.00"
  },
  {
    id: "M",
    name: "Michael Brown",
    email: "michael@example.com",
    type: "Individual",
    walletAddress: "ox4d5e6f7g8n8Mlohd",
    status: "Inactive",
    totalPaid: "$2,800.00"
  },
  {
    id: "TS",
    name: "Tech Solutions Inc.",
    email: "finance@techsolutions.com",
    type: "Company",
    walletAddress: "ox5e6f7g8h9d7vfa",
    status: "Active",
    totalPaid: "$32,100.00"
  },
  {
    id: "ED",
    name: "Emily Davis",
    email: "emily@example.com",
    type: "Individual",
    walletAddress: "oxe67g8h9d7jk4Sgu0",
    status: "Active",
    totalPaid: "$5,400.00"
  },
  {
    id: "GL",
    name: "Global Logistics Ltd.",
    email: "accounts@globallogistics.com",
    type: "Company",
    walletAddress: "ox7g8h9d7jk2iLkMd5",
    status: "Inactive",
    totalPaid: "$18,750.00"
  }
];
