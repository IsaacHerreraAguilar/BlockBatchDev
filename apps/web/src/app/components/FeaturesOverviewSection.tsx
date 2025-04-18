import Image from "next/image";


export default function FeaturesOverviewSection() {

    const features = [
        {
            title: "Batch Processing",
            icon: "/icons/processing-icon.svg",
            description: "Process thousands of payments in a single transaction, saving time and reducing costs."
        },
        {
            title: "Secure & Transparent",
            icon: "/icons/security-icon.svg",
            description: "Leverage blockchain security with complete transparency and immutable transaction records."
        },
        {
            title: "Real-time Analytics",
            icon: "/icons/analytics-icon.svg",
            description: "Monitor payment performance with detailed analytics and customizable reports."
        }
    ]





    return (
        <section className="w-full py-16 bg-[#F4F4F5] ">
            <div className="container mx-auto px-4 max-w-6xl">
                <div className="text-center flex flex-col items-center justify-center mb-16">
                    <div className="inline-block px-3 py-1 mb-4 text-sm font-normal bg-[#18181B] text-[#fafafa] rounded-md">Features</div>
                    <h2 className="text-3xl md:text-4xl font-bold text-[#000000] mb-4">Streamline Your Payment Operations</h2>
                    <p className="font-normal text-[#71717A] max-w-[803px] text-xl ">
                        BlockBatch combines the power of blockchain with intuitive design to revolutionize how businesses handle
                        payments.
                    </p>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-3 gap-8">





                    {
                        features.map((feature, index) => (
                            <div key={index} className="flex flex-col items-center text-center max-[324px]  ">
                                <div className="bg-[#18181B1A] w-20 h-20 rounded-full mb-6 flex items-center justify-center">
                                    <Image src={feature.icon} alt="icon" height={40} width={40} />
                                </div>
                                <h3 className="text-xl font-bold mb-3 text-[#000000] ">{feature.title}</h3>
                                <p className="text-[#71717A] text-base font-normal max ">
                                    {feature.description}
                                </p>
                            </div>
                        ))
                    }








                </div>
            </div>
        </section>
    )
}

