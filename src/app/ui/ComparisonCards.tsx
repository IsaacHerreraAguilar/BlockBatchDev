import React from "react";
interface Feature {
  icon: string;
  text: string;
  isPositive: boolean;
}

interface ComparisonCardProps {
  title: string;
  features: Feature[];
  className?: string;
}

const ComparisonCard: React.FC<ComparisonCardProps> = ({
  title,
  features,
  className,
}) => {
  return (
    <div
      className={`border border-[#E4E4E7] rounded-lg p-6 w-full max-w-md  ${className}`}
    >
      <div className="font-semibold  text-lg md:text-xl leading-7 flex items-center gap-2">
        <img src="/folderblack.svg" alt="category" />
        <h3 className="text-black">{title}</h3>
      </div>
      <ul className="mt-4 space-y-2">
        {features.map((feature, index) => (
          <li key={index} className="flex items-center gap-2 text-sm leading-5">
            <img src={feature.icon} alt="feature icon" className="" />
            {feature.text}
          </li>
        ))}
      </ul>
    </div>
  );
};

export default ComparisonCard;
