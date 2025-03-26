import { FC } from "react";
import BlockBatchFeatures from "../../components/ExploreBlockchain";

const MainPage: FC = () => {
  return (
    <main className="min-h-screen bg-gray-50 flex flex-col items-center justify-center">
      <BlockBatchFeatures />
    </main>
  );
};

export default MainPage;
