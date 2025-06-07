export interface RecipientsProps {
  id: string;
  name: string;
  email: string;
  type: "Individual" | "Company";
  walletAddress: string;
  status: string;
  totalPaid: string;
}
