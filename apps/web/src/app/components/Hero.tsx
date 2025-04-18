import Image from 'next/image';

const Hero = () => {
  return (
    <section className="relative bg-white py-16 md:py-24">
      <div className="container mx-auto px-4">
        <div className="flex flex-col lg:flex-row items-center justify-between gap-12">
          {/* Contenido de texto */}
          <div className="flex-1 max-w-2xl">
            <h1 className="text-4xl md:text-5xl lg:text-6xl font-bold leading-tight mb-6 text-[#111827]">
              Automate & Optimize Batch Payments with Blockchain
            </h1>
            <p className="text-gray-600 text-lg md:text-xl mb-8">
              BlockBatch streamlines your payment processes, reduces costs, and increases transparency with blockchain technology.
            </p>
            <div className="flex flex-col sm:flex-row gap-4">
              <button className="bg-black text-white px-8 py-3 rounded-md hover:bg-gray-800 transition-colors">
                Start Free Trial
              </button>
              <button className="border-2 border-gray-300 px-8 py-3 rounded-md hover:border-gray-400 transition-colors text-[#111827]">
                Request Demo
              </button>
            </div>
          </div>

          {/* Ilustración */}
          <div className="flex-1 flex justify-center items-center">
            <div className="relative w-full max-w-md">
              <Image
                src="/logo.png"
                alt="BlockBatch Logo"
                width={500}
                height={500}
                className="w-full h-auto"
                priority
              />
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default Hero; 