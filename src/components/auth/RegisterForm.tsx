import React, { useState } from 'react';
import { Input } from '../ui/Input';
import { Button } from '../ui/Button';
import { Checkbox } from '../ui/Checkbox';
import Link from 'next/link';
import Image from 'next/image';

interface FormData {
  fullName: string;
  email: string;
  password: string;
  confirmPassword: string;
  acceptTerms: boolean;
}

interface FormErrors {
  fullName?: string;
  email?: string;
  password?: string;
  confirmPassword?: string;
  acceptTerms?: string;
}

export const RegisterForm = () => {
  const [formData, setFormData] = useState<FormData>({
    fullName: '',
    email: '',
    password: '',
    confirmPassword: '',
    acceptTerms: false,
  });

  const [errors, setErrors] = useState<FormErrors>({});
  const [isLoading, setIsLoading] = useState(false);

  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};

    if (!formData.fullName) {
      newErrors.fullName = 'Full name is required';
    }

    if (!formData.email) {
      newErrors.email = 'Email is required';
    } else if (!/\S+@\S+\.\S+/.test(formData.email)) {
      newErrors.email = 'Email is invalid';
    }

    if (!formData.password) {
      newErrors.password = 'Password is required';
    } else if (formData.password.length < 8) {
      newErrors.password = 'Password must be at least 8 characters';
    }

    if (!formData.confirmPassword) {
      newErrors.confirmPassword = 'Please confirm your password';
    } else if (formData.password !== formData.confirmPassword) {
      newErrors.confirmPassword = 'Passwords do not match';
    }

    if (!formData.acceptTerms) {
      newErrors.acceptTerms = 'You must accept the Terms of Service and Privacy Policy';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!validateForm()) return;

  };

  const handleGoogleSignIn = () => {
    
  };

  return (
    <div className="min-h-screen flex flex-col py-12 container mx-auto px-4 sm:px-10 md:px-8 lg:px-16">
      <div className="flex items-start mb-8">
        <Link href="/" className="text-gray-500 hover:text-gray-700">
          ← Back to Home
        </Link>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-10">
        <div className="flex flex-col">
          <div>
            <h1 className="text-3xl font-bold text-gray-900">
              Create Your Account
            </h1>
            <p className="mt-2 text-sm text-gray-600">
              Join BlockBatch to streamline your batch payment processes
            </p>
          </div>

          <div className="mt-8">
            <form className="space-y-6" onSubmit={handleSubmit}>
              <Input
                label="Full Name"
                type="text"
                value={formData.fullName}
                onChange={(e) => setFormData({ ...formData, fullName: e.target.value })}
                error={errors.fullName}
                placeholder="John Doe"
              />

              <Input
                label="Email"
                type="email"
                value={formData.email}
                onChange={(e) => setFormData({ ...formData, email: e.target.value })}
                error={errors.email}
                placeholder="name@example.com"
              />

              <Input
                label="Password"
                type="password"
                value={formData.password}
                onChange={(e) => setFormData({ ...formData, password: e.target.value })}
                error={errors.password}
              />

              <Input
                label="Confirm Password"
                type="password"
                value={formData.confirmPassword}
                onChange={(e) => setFormData({ ...formData, confirmPassword: e.target.value })}
                error={errors.confirmPassword}
              />

              <Checkbox
                id="acceptTerms"
                checked={formData.acceptTerms}
                onChange={(e) => setFormData({ ...formData, acceptTerms: e.target.checked })}
                error={errors.acceptTerms}
                label={
                  <span>
                    I agree to the{' '}
                    <Link href="/terms" className="text-primary hover:text-primary/80">
                      Terms of Service
                    </Link>{' '}
                    and{' '}
                    <Link href="/privacy" className="text-primary hover:text-primary/80">
                      Privacy Policy
                    </Link>
                  </span>
                }
              />

              <Button type="submit" fullWidth isLoading={isLoading}>
                Create Account
              </Button>
            </form>

            <div className="mt-6">
              <div className="relative">
                <div className="absolute inset-0 flex items-center">
                  <div className="w-full border-t border-gray-300" />
                </div>
                <div className="relative flex justify-center text-sm">
                  <span className="px-2 bg-white text-gray-500">OR CONTINUE WITH</span>
                </div>
              </div>

              <Button
                type="button"
                variant="outline"
                fullWidth
                className="mt-6"
                onClick={handleGoogleSignIn}
              >
                <Image
                  src="/google.svg"
                  alt="Google"
                  width={20}
                  height={20}
                  className="mr-2"
                />
                Sign up with Google
              </Button>
            </div>

            <p className="mt-6 text-center text-sm text-gray-600">
              Already have an account?{' '}
              <Link href="/login" className="text-primary">
                Sign in
              </Link>
            </p>
          </div>
        </div>

        <div className="">
          <img src="/placeholder.svg" alt="Register" className="w-full h-[400px] object-cover" />
          <div className="bg-white py-8 px-4 border border-gray-200 rounded-b-lg sm:px-10">
            <h2 className="text-2xl font-bold text-gray-900">
              Why Choose BlockBatch?
            </h2>
            <p className="mt-2 text-gray-600">
              BlockBatch combines blockchain technology with intuitive design to revolutionize
              how businesses handle batch payments.
            </p>
            <ul className="mt-4 list-disc pl-4 space-y-3">
              <li className="text-gray-900">Process thousands of payments in minutes</li>
              <li className="text-gray-900">Reduce transaction costs by up to 80%</li>
              <li className="text-gray-900">Secure, transparent blockchain transactions</li>
              <li className="text-gray-900">Comprehensive analytics and reporting</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}; 