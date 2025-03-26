import { FC, ReactNode } from "react";
import Link from "next/link";
import { FileText, Plus, Settings, ChartBar } from "lucide-react";

interface FeatureCardProps {
  title: string;
  description: string;
  href: string;
  icon: ReactNode;
}

const FeatureCard: FC<FeatureCardProps> = ({
  title,
  description,
  href,
  icon,
}) => (
  <Link
    href={href}
    className="bg-white shadow-md p-6 rounded-lg flex flex-col items-center gap-4 hover:shadow-lg transition focus:outline-none focus:ring-2 focus:ring-gray-300"
  >
    <div className="text-gray-800 bg-[#E4E4E7] p-3 rounded-full">{icon}</div>
    <h3 className="font-bold text-lg text-black">{title}</h3>
    <p className="text-[#71717A] text-sm text-center">{description}</p>
  </Link>
);

const features = [
  {
    title: "Dashboard",
    description: "View your payment analytics and transaction summaries",
    href: "/Dashboard",
    icon: <ChartBar size={32} className="rotate-90 scale-x-[-1]" />,
  },
  {
    title: "Transactions",
    description: "Monitor and manage all your batch payment transactions",
    href: "/Transactions",
    icon: <FileText size={32} />,
  },
  {
    title: "Create Batch",
    description: "Set up and process new batch payments quickly",
    href: "/CreateBatch",
    icon: <Plus size={32} />,
  },
  {
    title: "Settings",
    description: "Configure your account, wallets, and API keys",
    href: "/SettingsPage",
    icon: <Settings size={32} />,
  },
];

const BlockBatchFeatures: FC = () => {
  return (
    <section className="bg-[#F4F4F5] py-12 px-6 text-center geist-sans">
      <h2 className="text-3xl text-black font-extrabold">
        Explore BlockBatch Features
      </h2>
      <p className="text-[#71717A] mt-2 mb-8 max-w-2xl mx-auto">
        Navigate through our platform to discover all the powerful features
        BlockBatch has to offer.
      </p>
      <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-6 max-w-6xl mx-auto">
        {features.map((feature, index) => (
          <FeatureCard key={index} {...feature} />
        ))}
      </div>
    </section>
  );
};

export default BlockBatchFeatures;
