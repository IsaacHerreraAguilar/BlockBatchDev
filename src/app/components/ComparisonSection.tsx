import React from "react";
import ComparisonCard from "../ui/ComparisonCards";

export default function ComparisonSection() {
  const blockchainFeatures = [
    {
      icon: "/dollargreen.svg",
      text: "Up to 80% lower transaction fees",
      isPositive: true,
    },
    {
      icon: "/dollargreen.svg",
      text: "Process thousands of payments in minutes",
      isPositive: true,
    },
    {
      icon: "/chartgreen.svg",
      text: "Immutable transaction records",
      isPositive: true,
    },
    {
      icon: "/timegreen.svg",
      text: "Real-time payment tracking",
      isPositive: true,
    },
    {
      icon: "/shieldgreen.svg",
      text: "Automated reconciliation",
      isPositive: true,
    },
  ];

  const bankingFeatures = [
    {
      icon: "/dollargray.svg",
      text: "High per-transaction fees",
      isPositive: false,
    },
    {
      icon: "/dollargray.svg",
      text: "Days to process batch payments",
      isPositive: false,
    },
    {
      icon: "/chartgray.svg",
      text: "Limited transaction visibility",
      isPositive: false,
    },
    {
      icon: "/timegray.svg",
      text: "Delayed payment confirmation",
      isPositive: false,
    },
    {
      icon: "/shieldgray.svg",
      text: "Manual reconciliation required",
      isPositive: false,
    },
  ];
  return (
    <div className="bg-white py-14 lg:py-36 px-4 ">
      <div className="flex flex-col gap-4  2xl:w-[1440px] 2xl:mx-auto">
        <div className=" flex flex-col gap-2 md:gap-3 2xl:gap-5  justify-center items-center">
          <div
            className=" max-w-28  bg-black mx-auto flex justify-center text-center
                    items-center px-3 py-2 rounded-lg text-sm leading-5"
          >
            <p className="text-[#FAFAFA] text-sm leading-5">Comparison</p>
          </div>
          <h2
            className=" text-2xl md:text-3xl lg:text-4xl text-center lg:leading-11
          2xl:text-6xl text-black font-bold "
          >
            BlockBatch vs. Traditional Banking
          </h2>
          <h5 className="text-[#71717A] text-sm  md:text-xl text-center md:leading-8 md:w-[80%] 2xl:text-3xl 2xl:w-3/4">
            See how BlockBatch outperforms traditional payment methods across
            key metrics.
          </h5>
        </div>

        <div className="flex flex-col md:flex-row gap-6 justify-center  md:p-10">
          <ComparisonCard
            className="text-black mx-auto lg:mx-0"
            title="BlockBatch Blockchain"
            features={blockchainFeatures}
          />
          <ComparisonCard
            title="Traditional Banking"
            features={bankingFeatures}
            className="text-[#71717A] mx-auto lg:mx-0"
          />
        </div>
      </div>
    </div>
  );
}
