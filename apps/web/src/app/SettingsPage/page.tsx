"use client";

import { useState } from "react";
import { Copy, Trash2, Plus } from "lucide-react";

const TABS = [
  { label: "Profile" },
  { label: "Notifications" },
  { label: "Wallets" },
  { label: "API Keys" },
];

type FormState = {
  fullName: string;
  email: string;
  phone: string;
  timezone: string;
  company: string;
  website: string;
  address: string;
};

type FormErrors = Partial<Record<keyof FormState, string>>;

export default function Settings() {
  const [activeTab, setActiveTab] = useState("Profile");
  const [twoFactor, setTwoFactor] = useState(false);
  const [sessionTimeout, setSessionTimeout] = useState(true);
  const [showModal, setShowModal] = useState(false);

  // Estados para los campos del formulario
  const [form, setForm] = useState<FormState>({
    fullName: "",
    email: "",
    phone: "",
    timezone: "",
    company: "",
    website: "",
    address: "",
  });
  const [errors, setErrors] = useState<FormErrors>({});
  const [submitted, setSubmitted] = useState(false);

  function validate(form: FormState): FormErrors {
    const newErrors: FormErrors = {};
    if (!form.fullName.trim()) newErrors.fullName = "This field is required";
    if (!form.email.trim()) newErrors.email = "This field is required";
    else if (!/^\S+@\S+\.\S+$/.test(form.email)) newErrors.email = "Please enter a valid email address";
    if (!form.phone.trim()) newErrors.phone = "This field is required";
    if (!form.timezone.trim()) newErrors.timezone = "This field is required";
    if (!form.company.trim()) newErrors.company = "This field is required";
    if (!form.website.trim()) newErrors.website = "This field is required";
    if (!form.address.trim()) newErrors.address = "This field is required";
    return newErrors;
  }

  function handleChange(e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) {
    setForm({ ...form, [e.target.name]: e.target.value });
  }

  function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    setSubmitted(true);
    const validation = validate(form);
    setErrors(validation);
    if (Object.keys(validation).length === 0) {
      setShowModal(true);
    }
  }

  function handleModalClose() {
    setShowModal(false);
    setForm({
      fullName: "",
      email: "",
      phone: "",
      timezone: "",
      company: "",
      website: "",
      address: "",
    });
    setSubmitted(false);
    setErrors({});
  }

  return (
    <main className="min-h-screen bg-gray-50 flex flex-col items-center py-12">
      <div className="w-full max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
        <h1
          className="mb-2"
          style={{
            fontFamily: 'Geist, sans-serif',
            fontWeight: 400,
            fontSize: '36px',
            lineHeight: '40px',
            color: '#3F3F46',
            letterSpacing: 0,
            verticalAlign: 'middle',
          }}
        >
          Settings
        </h1>
        <p
          className="mb-6"
          style={{
            fontFamily: 'Geist, sans-serif',
            fontWeight: 400,
            fontSize: '18px',
            lineHeight: '28px',
            color: '#71717A',
            letterSpacing: 0,
            verticalAlign: 'middle',
          }}
        >
          Manage your account settings and preferences
        </p>
        <div className="mb-6">
          <nav
            className="flex bg-gray-100 rounded-lg p-1 w-fit gap-1"
            aria-label="Tabs"
          >
            {TABS.map((tab) => (
              <button
                key={tab.label}
                onClick={() => setActiveTab(tab.label)}
                style={{
                  fontFamily: 'Geist, sans-serif',
                  fontWeight: 500,
                  fontSize: '14px',
                  lineHeight: '20px',
                  letterSpacing: 0,
                  textAlign: 'center',
                  verticalAlign: 'middle',
                  color: activeTab === tab.label ? '#09090B' : '#71717A',
                }}
                className={`px-5 py-2 rounded-md transition-all focus:outline-none
                  ${
                    activeTab === tab.label
                      ? "bg-white shadow font-semibold"
                      : "bg-transparent hover:text-black"
                  }
                `}
              >
                {tab.label}
              </button>
            ))}
          </nav>
        </div>
       <div
          //className="bg-white rounded-lg shadow p-8 border border-gray-200"
          style={{ width: '920.56px', margin: '0 auto' }}
        >
          {activeTab === "Profile" && (
            <div className="bg-white rounded-lg shadow p-6">
              {/* Profile Form */}
              <form className="space-y-8" onSubmit={handleSubmit} noValidate>
                {/* Personal Information */}
                <div>
                  <h2 className="text-lg font-semibold mb-1" style={{fontFamily: 'Geist, sans-serif', color: '#18181B'}}>Profile</h2>
                  <p className="text-sm mb-4" style={{fontFamily: 'Geist, sans-serif', color: '#71717A'}}>Manage your personal information and company details</p>
                  <h3 className="mb-2" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '18px', lineHeight: '28px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Personal Information</h3>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                    <div>
                      <label className="block mb-1" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '14px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Full Name</label>
                      <input type="text" name="fullName" value={form.fullName} onChange={handleChange} placeholder="John Doe" className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300" style={{fontFamily: 'Geist, sans-serif', fontWeight: 400, fontSize: '14px', lineHeight: '100%', letterSpacing: 0, verticalAlign: 'middle'}} />
                      {submitted && errors.fullName && <span className="text-red-500 text-xs mt-1 block">{errors.fullName}</span>}
                    </div>
                    <div>
                      <label className="block mb-1" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '14px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Email</label>
                      <input type="email" name="email" value={form.email} onChange={handleChange} placeholder="john@example.com" className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300" style={{fontFamily: 'Geist, sans-serif', fontWeight: 400, fontSize: '14px', lineHeight: '100%', letterSpacing: 0, verticalAlign: 'middle'}} />
                      {submitted && errors.email && <span className="text-red-500 text-xs mt-1 block">{errors.email}</span>}
                    </div>
                    <div>
                      <label className="block mb-1" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '14px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Phone Number</label>
                      <input type="tel" name="phone" value={form.phone} onChange={handleChange} placeholder="+1 (555) 123-4567" className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300" style={{fontFamily: 'Geist, sans-serif', fontWeight: 400, fontSize: '14px', lineHeight: '100%', letterSpacing: 0, verticalAlign: 'middle'}} />
                      {submitted && errors.phone && <span className="text-red-500 text-xs mt-1 block">{errors.phone}</span>}
                    </div>
                    <div>
                      <label className="block mb-1" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '14px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Timezone</label>
                      <input type="text" name="timezone" value={form.timezone} onChange={handleChange} placeholder="America/New_York" className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300" style={{fontFamily: 'Geist, sans-serif', fontWeight: 400, fontSize: '14px', lineHeight: '100%', letterSpacing: 0, verticalAlign: 'middle'}} />
                      {submitted && errors.timezone && <span className="text-red-500 text-xs mt-1 block">{errors.timezone}</span>}
                    </div>
                  </div>
                </div>
                {/* Company Information */}
                <div>
                  <h3 className="mb-2" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '18px', lineHeight: '28px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Company Information</h3>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                    <div>
                      <label className="block mb-1" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '14px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Company Name</label>
                      <input type="text" name="company" value={form.company} onChange={handleChange} placeholder="Acme Inc." className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300" style={{fontFamily: 'Geist, sans-serif', fontWeight: 400, fontSize: '14px', lineHeight: '100%', letterSpacing: 0, verticalAlign: 'middle'}} />
                      {submitted && errors.company && <span className="text-red-500 text-xs mt-1 block">{errors.company}</span>}
                    </div>
                    <div>
                      <label className="block mb-1" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '14px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Website</label>
                      <input type="url" name="website" value={form.website} onChange={handleChange} placeholder="https://acme.com" className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300" style={{fontFamily: 'Geist, sans-serif', fontWeight: 400, fontSize: '14px', lineHeight: '100%', letterSpacing: 0, verticalAlign: 'middle'}} />
                      {submitted && errors.website && <span className="text-red-500 text-xs mt-1 block">{errors.website}</span>}
                    </div>
                  </div>
                  <div>
                    <label className="block mb-1" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '14px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Address</label>
                    <textarea
                      placeholder="123 Main St, Suite 100, New York, NY 10001"
                      rows={3}
                      name="address"
                      value={form.address}
                      onChange={handleChange}
                      className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300 resize-none"
                      style={{fontFamily: 'Geist, sans-serif', fontWeight: 400, fontSize: '14px', lineHeight: '100%', letterSpacing: 0, verticalAlign: 'middle'}}
                    />
                    {submitted && errors.address && <span className="text-red-500 text-xs mt-1 block">{errors.address}</span>}
                  </div>
                </div>
                {/* Security */}
                <div>
                  <h3 className="mb-2" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '18px', lineHeight: '28px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Security</h3>
                  <div className="flex flex-col gap-4">
                    <div className="flex items-center justify-between">
                      <div>
                        <label className="block" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '14px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Two-Factor Authentication</label>
                        <span className="block" style={{fontFamily: 'Geist, sans-serif', fontWeight: 400, fontSize: '14px', lineHeight: '20px', letterSpacing: 0, verticalAlign: 'middle', color: '#71717A'}}>Add an extra layer of security to your account</span>
                      </div>
                      {/* Switch para Two-Factor Authentication */}
                      <button
                        type="button"
                        aria-pressed={twoFactor}
                        onClick={() => setTwoFactor((v) => !v)}
                        className={`w-11 h-6 rounded-full transition-colors duration-200 flex items-center focus:outline-none shadow-sm
                          ${twoFactor ? 'bg-[#18181B]' : 'bg-[#E4E4E7]'}
                        `}
                        style={{ minWidth: 44 }}
                      >
                        <span
                          className={`inline-block w-5 h-5 bg-white rounded-full shadow transform transition-transform duration-200
                            ${twoFactor ? 'translate-x-5' : 'translate-x-1'}
                          `}
                        />
                      </button>
                    </div>
                    <div className="flex items-center justify-between">
                      <div>
                        <label className="block" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '14px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Session Timeout</label>
                        <span className="block" style={{fontFamily: 'Geist, sans-serif', fontWeight: 400, fontSize: '14px', lineHeight: '20px', letterSpacing: 0, verticalAlign: 'middle', color: '#71717A'}}>Automatically log out after period of inactivity</span>
                      </div>
                      {/* Switch para Session Timeout */}
                      <button
                        type="button"
                        aria-pressed={sessionTimeout}
                        onClick={() => setSessionTimeout((v) => !v)}
                        className={`w-11 h-6 rounded-full transition-colors duration-200 flex items-center focus:outline-none shadow-sm
                          ${sessionTimeout ? 'bg-[#18181B]' : 'bg-[#E4E4E7]'}
                        `}
                        style={{ minWidth: 44 }}
                      >
                        <span
                          className={`inline-block w-5 h-5 bg-white rounded-full shadow transform transition-transform duration-200
                            ${sessionTimeout ? 'translate-x-5' : 'translate-x-1'}
                          `}
                        />
                      </button>
                    </div>
                  </div>
                </div>
                <div className="flex justify-start">
                  <button
                    type="submit"
                    style={{
                      background: '#18181B',
                      fontFamily: 'Geist, sans-serif',
                      fontWeight: 500,
                      fontSize: '14px',
                      lineHeight: '20px',
                      letterSpacing: 0,
                      textAlign: 'center',
                      verticalAlign: 'middle',
                      color: '#FAFAFA',
                    }}
                    className="px-6 py-2 rounded-md transition hover:bg-gray-900"
                  >
                    Save Changes
                  </button>
                </div>
              </form>
            </div>
          )}
          {activeTab === "Wallets" && (
            <div className="bg-white rounded-lg shadow p-6">
              <h2
                style={{
                  fontFamily: 'Geist',
                  fontWeight: 600,
                  fontSize: '24px',
                  lineHeight: '24px',
                  letterSpacing: '-0.6px',
                  verticalAlign: 'middle',
                  color: '#09090B',
                }}
                className="mb-1"
              >
                Blockchain Wallets
              </h2>
              <p
                style={{
                  fontFamily: 'Geist',
                  fontWeight: 400,
                  fontSize: '14px',
                  lineHeight: '20px',
                  letterSpacing: '0%',
                  verticalAlign: 'middle',
                  color: '#71717A',
                }}
                className="mb-6"
              >
                Manage your connected blockchain wallets for batch payments
              </p>
              <div className="flex justify-between items-center mb-4">
                <h3
                  style={{
                    fontFamily: 'Geist',
                    fontWeight: 500,
                    fontSize: '18px',
                    lineHeight: '28px',
                    letterSpacing: '0%',
                    verticalAlign: 'middle',
                    color: '#09090B',
                  }}
                >Connected Wallets</h3>
                <button
                  style={{
                    fontFamily: 'Geist',
                    fontWeight: 500,
                    fontSize: '14px',
                    lineHeight: '20px',
                    letterSpacing: '0%',
                    textAlign: 'center',
                    verticalAlign: 'middle',
                    color: '#FAFAFA',
                    backgroundColor: '#18181B',
                  }}
                  className="px-4 py-2 rounded-md text-sm font-medium flex items-center gap-1 hover:opacity-90 transition-opacity"
                >
                  <Plus size={16} />
                  Add Wallet
                </button>
              </div>
              <div className="overflow-x-auto mb-8">
                <table className="w-full text-sm border-separate border-spacing-0 border border-gray-200 rounded-md overflow-hidden">
                  <thead>
                    <tr>
                      <th
                        className="text-left px-4 py-2"
                        style={{
                          fontFamily: 'Geist',
                          fontWeight: 500,
                          fontSize: '14px',
                          lineHeight: '20px',
                          letterSpacing: '0%',
                          verticalAlign: 'middle',
                          color: '#71717A',
                        }}
                      >
                        Name
                      </th>
                      <th
                        className="text-left px-4 py-2"
                        style={{
                          fontFamily: 'Geist',
                          fontWeight: 500,
                          fontSize: '14px',
                          lineHeight: '20px',
                          letterSpacing: '0%',
                          verticalAlign: 'middle',
                          color: '#71717A',
                        }}
                      >
                        Wallet Address
                      </th>
                      <th
                        className="text-left px-4 py-2"
                        style={{
                          fontFamily: 'Geist',
                          fontWeight: 500,
                          fontSize: '14px',
                          lineHeight: '20px',
                          letterSpacing: '0%',
                          verticalAlign: 'middle',
                          color: '#71717A',
                        }}
                      >
                        Network
                      </th>
                      <th
                        className="text-left px-4 py-2"
                        style={{
                          fontFamily: 'Geist',
                          fontWeight: 500,
                          fontSize: '14px',
                          lineHeight: '20px',
                          letterSpacing: '0%',
                          verticalAlign: 'middle',
                          color: '#71717A',
                        }}
                      >
                        Status
                      </th>
                      <th
                        className="text-left px-4 py-2"
                        style={{
                          fontFamily: 'Geist',
                          fontWeight: 500,
                          fontSize: '14px',
                          lineHeight: '20px',
                          letterSpacing: '0%',
                          verticalAlign: 'middle',
                          color: '#71717A',
                        }}
                      >
                        Actions
                      </th>
                    </tr>
                  </thead>
                  <tbody style={{fontFamily: 'Geist', fontSize: '14px'}}>
                    {[
                      {
                        name: 'Main Company Wallet',
                        address: '0x1a2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8r9s0t',
                        network: 'Ethereum',
                        status: 'Default',
                      },
                      {
                        name: 'Payroll Wallet',
                        address: '0x9s8r7q6p5o4n3m2l1k0j9i8h7g6f5e4d3c2b1a0',
                        network: 'Polygon',
                        status: 'Connected',
                      },
                      {
                        name: 'Vendor Payments',
                        address: '0x2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8r9s0t1',
                        network: 'Optimism',
                        status: 'Connected',
                      },
                    ].map((wallet) => (
                      <tr key={wallet.name} className="border-b last:border-b-0">
                        <td
                          className="px-4 py-3 whitespace-nowrap"
                          style={{
                            fontFamily: 'Geist',
                            fontWeight: 500,
                            fontSize: '14px',
                            lineHeight: '20px',
                            letterSpacing: '0%',
                            verticalAlign: 'middle',
                            color: '#09090B',
                          }}
                        >
                          {wallet.name}
                        </td>
                        <td className="px-4 py-3 flex items-center gap-2 whitespace-nowrap">
                          <span
                            className="truncate max-w-[160px]"
                            title={wallet.address}
                            style={{
                              fontFamily: 'Inter',
                              fontWeight: 400,
                              fontSize: '11.81px',
                              lineHeight: '16px',
                              letterSpacing: '0%',
                              verticalAlign: 'middle',
                              color: '#09090B',
                            }}
                          >
                            {wallet.address}
                          </span>
                          <button className="p-1 hover:bg-gray-100 rounded" title="Copy address">
                            <Copy size={16} className="text-gray-400" />
                          </button>
                        </td>
                        <td
                          className="px-4 py-3 whitespace-nowrap"
                          style={{
                            fontFamily: 'Geist',
                            fontWeight: 400,
                            fontSize: '14px',
                            lineHeight: '20px',
                            letterSpacing: '0%',
                            verticalAlign: 'middle',
                            color: '#09090B',
                          }}
                        >
                          {wallet.network}
                        </td>
                        <td className="px-4 py-3 whitespace-nowrap">
                          {wallet.status === 'Default' ? (
                            <span
                              className="bg-green-50 text-green-600 px-3 py-1 rounded-full text-xs font-medium"
                              style={{
                                fontFamily: 'Geist',
                                fontWeight: 600,
                                fontSize: '12px',
                                lineHeight: '16px',
                                letterSpacing: '0%',
                                verticalAlign: 'middle',
                              }}
                            >
                              Default
                            </span>
                          ) : (
                            <span
                              className="bg-gray-100 text-gray-600 px-3 py-1 rounded-full text-xs font-medium"
                              style={{
                                fontFamily: 'Geist',
                                fontWeight: 600,
                                fontSize: '12px',
                                lineHeight: '16px',
                                letterSpacing: '0%',
                                verticalAlign: 'middle',
                              }}
                            >
                              Connected
                            </span>
                          )}
                        </td>
                        <td className="px-4 py-3 flex gap-2 whitespace-nowrap">
                          <button
                            className="border border-gray-200 rounded px-3 py-1 text-xs font-medium hover:bg-gray-50"
                            style={{
                              fontFamily: 'Geist',
                              fontWeight: 500,
                              fontSize: '14px',
                              lineHeight: '20px',
                              letterSpacing: '0%',
                              textAlign: 'center',
                              verticalAlign: 'middle',
                              color: '#09090B',
                            }}
                          >
                            Edit
                          </button>
                          {wallet.status !== 'Default' && (
                            <button
                              className="border border-gray-200 rounded px-3 py-1 text-xs font-medium hover:bg-gray-50"
                              style={{
                                fontFamily: 'Geist',
                                fontWeight: 500,
                                fontSize: '14px',
                                lineHeight: '20px',
                                letterSpacing: '0%',
                                textAlign: 'center',
                                verticalAlign: 'middle',
                                color: '#09090B',
                              }}
                            >
                              Set Default
                            </button>
                          )}
                          {wallet.status !== 'Default' && (
                            <button className="p-1 hover:bg-gray-100 rounded" title="Delete wallet">
                              <Trash2 size={16} className="text-red-400" />
                            </button>
                          )}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
              <div>
                <h3
                  style={{
                    fontFamily: 'Geist',
                    fontWeight: 500,
                    fontSize: '18px',
                    lineHeight: '28px',
                    letterSpacing: '0%',
                    verticalAlign: 'middle',
                    color: '#09090B',
                  }}
                  className="mb-4"
                >Add New Wallet</h3>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                  <div>
                    <label
                      className="block text-sm font-medium text-gray-700 mb-1"
                      style={{
                        fontFamily: 'Geist',
                        fontWeight: 500,
                        fontSize: '14px',
                        lineHeight: '14px',
                        letterSpacing: '0%',
                        verticalAlign: 'middle',
                        color: '#09090B',
                      }}
                    >
                      Wallet Name
                    </label>
                    <input
                      type="text"
                      placeholder="e.g., Marketing Expenses"
                      className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300"
                      style={{
                        fontFamily: 'Geist',
                        fontWeight: 400,
                        fontSize: '14px',
                        lineHeight: '100%',
                        letterSpacing: '0%',
                        verticalAlign: 'middle',
                        color: '#71717A',
                      }}
                    />
                  </div>
                  <div>
                    <label
                      className="block text-sm font-medium text-gray-700 mb-1"
                      style={{
                        fontFamily: 'Geist',
                        fontWeight: 500,
                        fontSize: '14px',
                        lineHeight: '14px',
                        letterSpacing: '0%',
                        verticalAlign: 'middle',
                        color: '#09090B',
                      }}
                    >
                      Network
                    </label>
                    <input
                      type="text"
                      placeholder="e.g., Ethereum, Polygon"
                      className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300"
                      style={{
                        fontFamily: 'Geist',
                        fontWeight: 400,
                        fontSize: '14px',
                        lineHeight: '100%',
                        letterSpacing: '0%',
                        verticalAlign: 'middle',
                        color: '#71717A',
                      }}
                    />
                  </div>
                </div>
                <div className="mb-6">
                  <label
                    className="block text-sm font-medium text-gray-700 mb-1"
                    style={{
                      fontFamily: 'Geist',
                      fontWeight: 500,
                      fontSize: '14px',
                      lineHeight: '14px',
                      letterSpacing: '0%',
                      verticalAlign: 'middle',
                      color: '#09090B',
                    }}
                  >
                    Wallet Address
                  </label>
                  <input
                    type="text"
                    placeholder="0x..."
                    className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300"
                    style={{
                      fontFamily: 'Geist',
                      fontWeight: 400,
                      fontSize: '14px',
                      lineHeight: '100%',
                      letterSpacing: '0%',
                      verticalAlign: 'middle',
                      color: '#71717A',
                    }}
                  />
                </div>
                <div className="flex justify-between items-center">
                  <button
                    style={{
                      fontFamily: 'Geist',
                      fontWeight: 500,
                      fontSize: '14px',
                      lineHeight: '20px',
                      letterSpacing: '0%',
                      textAlign: 'center',
                      verticalAlign: 'middle',
                      color: '#09090B',
                    }}
                    className="px-4 py-2 rounded-md text-sm font-medium bg-white border border-gray-200 hover:bg-gray-100 transition-colors"
                  >
                    Cancel
                  </button>
                  <button
                    style={{
                      fontFamily: 'Geist',
                      fontWeight: 500,
                      fontSize: '14px',
                      lineHeight: '20px',
                      letterSpacing: '0%',
                      textAlign: 'center',
                      verticalAlign: 'middle',
                      color: '#FAFAFA',
                      backgroundColor: '#18181B',
                    }}
                    className="px-4 py-2 rounded-md text-sm font-medium hover:opacity-90 transition-opacity"
                  >
                    Save Wallet
                  </button>
                </div>
              </div>
            </div>
          )}
          {activeTab === "Notifications" && (
            <div className="bg-white rounded-lg shadow p-6 flex items-center justify-center min-h-[200px]">
              <span className="text-gray-400 text-lg font-medium">Coming soon...</span>
            </div>
          )}
          {activeTab === "API Keys" && (
            <div className="bg-white rounded-lg shadow p-6 flex items-center justify-center min-h-[200px]">
              <span className="text-gray-400 text-lg font-medium">Coming soon...</span>
            </div>
          )}
        </div>
        {/* Modal de éxito */}
        {showModal && (
          <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/10 backdrop-blur-sm">
            <div className="bg-white rounded-lg shadow-lg p-8 max-w-sm w-full flex flex-col items-center">
              <span className="text-2xl mb-2" role="img" aria-label="success">✅</span>
              <h2 className="text-lg font-semibold mb-2" style={{fontFamily: 'Geist, sans-serif', color: '#18181B'}}>Profile saved successfully!</h2>
              <button
                onClick={handleModalClose}
                className="mt-4 px-6 py-2 rounded-md bg-black text-white font-medium hover:bg-gray-900 transition"
                style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '20px', letterSpacing: 0, textAlign: 'center', verticalAlign: 'middle'}}
              >
                OK
              </button>
            </div>
          </div>
        )}
      </div>
    </main>
  );
}
