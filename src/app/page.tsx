import FeaturesOverviewSection from "./components/FeaturesOverviewSection";
import { ProductDemo } from "@/components/product-demo";
import FinalCtaSection from "./components/FinalCtaSection";

export default function Home() {
  return (
    <div>
      <FeaturesOverviewSection />
      <ProductDemo />
      <FinalCtaSection />
    </div>
  );
}
