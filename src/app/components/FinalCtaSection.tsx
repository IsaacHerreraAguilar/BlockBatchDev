import React from "react";
import { Button } from "../ui/Button";
import { ArrowRight } from "lucide-react";

export default function FinalCtaSection() {
  return (
    <div className="bg-[#F4F4F5] px-4 min-[480px]:px-8 sm:px-10 py-20 md:py-24 lg:py-44 flex justify-center items-center">
      <div className="flex flex-col gap-4 text-center max-w-[1440px] mx-auto 2xl:gap-8">
        <h1 className="text-[28px] sm:text-3xl leading-9 text-[#000000] md:text-4xl md:leading-11 tracking-tight font-bold 2xl:text-6xl">
          Ready to Transform Your Payment Process?
        </h1>
        <h3 className="text-sm leading-6 sm:text-base md:text-lg text-[#71717A] sm:leading-8 lg:text-xl font-normal min-[400px]:w-[70%] mx-auto 2xl:text-3xl 2xl:leading-11">
          Join thousands of businesses already saving time and money with
          BlockBatch.
        </h3>
        <div className="flex flex-col min-[400px]:flex-row gap-4 justify-center">
          <Button
            type="button"
            className="bg-[#18181B] py-3 px-8 flex gap-4 text-sm font-medium 2xl:text-2xl 2xl:py-5 2xl:px-12 
            transition-all duration-300 ease-in-out hover:bg-[#2D2D30] 
            focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-[#18181B]"
          >
            Get Started <ArrowRight />
          </Button>

          <Button
            type="button"
            variant="outline"
            className="bg-[#ffffff] border border-[#E4E4E7] text-black text-sm font-medium py-3 
            px-8 2xl:text-2xl 2xl:py-5 2xl:px-12 transition-all duration-300 ease-in-out 
            hover:bg-[#F4F4F5] 
            focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-[#E4E4E7]"
          >
            Request Demo
          </Button>
        </div>
      </div>
    </div>
  );
}
