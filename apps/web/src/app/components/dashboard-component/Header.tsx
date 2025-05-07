import React from "react";

const Header = () => {
  return (
    <div className="w-full px-4 md:px-6 lg:px-0">
      <div className="flex flex-col sm:flex-row max-w-[1171px] w-full mx-auto items-start sm:items-center justify-between border-b pb-4 gap-4">
        <div>
          <h1 className="text-2xl font-semibold text-gray-800">Dashboard</h1>
          <p className="text-sm text-gray-500">Overview of your batch payment activities</p>
        </div>
        <button className="bg-black text-white text-sm px-4 py-2 rounded hover:bg-gray-800">
          + Create Batch
        </button>
      </div>
    </div>
  );
};

export default Header;
