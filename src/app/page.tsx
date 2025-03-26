import ComparisonSection from "./components/ComparisonSection";
import Hero from './components/Hero';
import FeaturesOverviewSection from "./components/FeaturesOverviewSection";
import { ProductDemo } from "@/app/components/product-demo";
import FinalCtaSection from "./components/FinalCtaSection";
import BlockBatchFeatures from "@/app/components/ExploreBlockchain";

export default function Home() {
  return (
    <div>
      <Hero />
      <FeaturesOverviewSection />
      <ComparisonSection />
      <BlockBatchFeatures />
      <ProductDemo />
      <FinalCtaSection />
    </div>
  );
}
