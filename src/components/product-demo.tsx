import Image from "next/image";
import logo from '../../public/placeholder.png';
import { ArrowRight, CircleCheckBig } from "lucide-react";


const PRODUCT_BENEFITS = ["Real-time transaction monitoring", "Intuitive batch payment creation", "Comprehensive analytics and reporting"]
export function ProductDemo() {
    return (
        <section className="mx-auto container px-4 md:px-12 py-3  flex flex-col md:flex-row  justify-between items-center">
            <div className="py-6 max-w-lg">
                <div className="inline-block bg-gray-900 text-white text-sm px-3 py-1 rounded-md mb-2">
                    Interactive Demo
                </div>

                <h1 className="text-3xl font-bold mb-3">Experience BlockBatch in Action</h1>

                <p className="text-gray-600 mb-6">
                    See how easy it is to manage batch payments with our intuitive
                    dashboard and powerful features.
                </p>

                <div className="space-y-3 mb-8">
                    {PRODUCT_BENEFITS.map((val, index) => <ProductListItem key={`${index}_${Math.random()}`} value={val} />)}


                </div>

                <div className="flex flex-col md:flex-row gap-4">
                    <button className="bg-gray-900 text-white justify-center px-4 py-3  gap-4 rounded-md hover:scale-102 active:scale-98 flex items-center">
                        Try Demo
                        <ArrowRight size={16} />
                    </button>

                    <button className="text-gray-900 px-4 py-3 border border-gray-300 rounded-md hover:scale-102 active:scale-98">
                        Request Personalized Demo
                    </button>
                </div>
            </div>
            <div className="relative h-fit w-fit shadow-2xs rounded-lg  overflow-hidden">
                <Image src={logo} alt="" width={550} height={550} />
                <div className="absolute inset-0 bg-linear-to-t from-black/20 to-transparent"></div>
                <div className="flex gap-2 absolute bottom-2 ml-2 font-medium text-sm">
                    <button className="bg-primary rounded-md px-2 h-9 text-white cursor-pointer hover:scale-102 active:scale-98">Open Dashboard</button>
                    <button className="bg-white rounded-md px-2 h-9 cursor-pointer hover:scale-102 active:scale-98">Learn More</button>
                </div>
            </div>
        </section>
    )
}

function ProductListItem({ value }: { value: string; icon?: React.ReactNode }) {
    return (
        <div className="flex items-center">
            <div className="bg-gray-100 rounded-full p-1 mr-3">
                <CircleCheckBig size={18} className="text-gray-600" />
            </div>
            <span className="text-gray-600">{value}</span>
        </div>
    )
}