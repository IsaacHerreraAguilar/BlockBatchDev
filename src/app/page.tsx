import Hero from './components/Hero';
import FeaturesOverviewSection from "./components/FeaturesOverviewSection";
import { ProductDemo } from "@/components/product-demo";
import FinalCtaSection from "./components/FinalCtaSection";

export default function Home() {
  return (
    <div>
      <Hero />
      <FeaturesOverviewSection />
      <ProductDemo />
      <FinalCtaSection />
    </div>
  );
}
