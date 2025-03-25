import { Button } from '@/components/ui/Button';
import { Checkbox } from '@/components/ui/Checkbox';
import { Input } from '@/components/ui/Input';
import Image from 'next/image';
import Link from 'next/link';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { BsArrowLeftShort } from 'react-icons/bs';
import { useEffect } from 'react';

const loginFormSchema = z.object({
  email: z.string().email('Email is invalid'),
  password: z.string().min(8, 'Password must be at least 8 characters'),
  rememberMe: z.boolean().optional(),
});

type LoginFormProps = z.infer<typeof loginFormSchema>;

export default function LoginForm() {
  const {
    register,
    handleSubmit,
    setValue,
    formState: { errors, isValid },
  } = useForm<LoginFormProps>({
    resolver: zodResolver(loginFormSchema),
    mode: 'onChange',
  });

  const onSubmit = (data: LoginFormProps) => {
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const { rememberMe, ...otherLoginData } = data;
    const loginData = otherLoginData;

    console.log(loginData);
    if (data.rememberMe) {
      localStorage.setItem('email', loginData.email);
    } else {
      localStorage.removeItem('email');
    }
    // Login request here
  };

  useEffect(() => {
    const storedEmail = localStorage.getItem('email');
    if (storedEmail) {
      setValue('email', storedEmail);
    }
  }, [setValue]);

  return (
    <div className='min-h-screen flex flex-col py-12 xl:max-w-[1600px] mx-auto px-4 sm:px-10 md:px-8 lg:px-16'>
      <div className='flex items-start mb-8'>
        <Link
          href='/'
          className='text-gray-500 hover:text-gray-700 flex space-x-2 items-center'
        >
          <BsArrowLeftShort className='w-5 h-5' />
          <span className='text-sm'>Back to Home</span>
        </Link>
      </div>

      <div className='grid grid-cols-1 md:grid-cols-2 gap-10'>
        <div className='flex flex-col'>
          <div>
            <h1 className='text-3xl font-bold text-gray-900'>Welcome Back</h1>
            <p className='mt-2 text-sm text-gray-600'>
              Sign in to your BlockBatch account to manage your batch payments
            </p>
          </div>

          <div className='mt-8'>
            <form className='space-y-6' onSubmit={handleSubmit(onSubmit)}>
              <Input
                label='Email'
                type='email'
                {...register('email')}
                error={errors.email?.message}
                placeholder='name@example.com'
              />

              <Input
                label='Password'
                labelComponent={
                  <div className='flex justify-between'>
                    <label
                      className='text-sm font-medium text-gray-700'
                      htmlFor='password'
                    >
                      Password
                    </label>
                    <Link
                      href='/forgot-password'
                      className='text-sm text-gray-700'
                    >
                      Forgot password?
                    </Link>
                  </div>
                }
                type='password'
                {...register('password')}
                error={errors.password?.message}
              />

              <Checkbox
                id='rememberMe'
                {...register('rememberMe')}
                error={errors.rememberMe?.message}
                label='Remember Me'
              />

              <Button type='submit' fullWidth disabled={!isValid}>
                Sign in
              </Button>
            </form>

            <div className='mt-6'>
              <div className='relative'>
                <div className='absolute inset-0 flex items-center'>
                  <div className='w-full border-t border-gray-300' />
                </div>
                <div className='relative flex justify-center text-sm'>
                  <span className='px-2 bg-white text-gray-500'>
                    OR CONTINUE WITH
                  </span>
                </div>
              </div>

              <Button
                type='button'
                variant='outline'
                fullWidth
                className='mt-6'
              >
                <Image
                  src='/images/google.svg'
                  alt='Google'
                  width={20}
                  height={20}
                  className='mr-2'
                />
                Sign in with Google
              </Button>
            </div>

            <p className='mt-6 text-center text-sm text-gray-600'>
              {`Don't have an account ?`}{' '}
              <Link href='/register' className='text-primary'>
                Sign up
              </Link>
            </p>
          </div>
        </div>

        <div>
          <Image
            width={100}
            height={100}
            src='/images/auth_placeholder.svg'
            alt='Register'
            className='w-full h-[350px] object-cover rounded-t-lg'
          />
          <div className='bg-white py-8 px-4 border border-gray-200 rounded-b-lg sm:px-10'>
            <h2 className='text-2xl font-bold text-gray-900'>
              Streamline Your Batch Payments
            </h2>
            <p className='mt-2 text-gray-600'>
              BlockBatch helps you process thousands of payments in minutes,
              reducing costs by up to 80% compared to traditional methods.
            </p>
            <ul className='mt-4 list-disc pl-4 space-y-3'>
              <li className='text-gray-900'>Secure blockchain transactions</li>
              <li className='text-gray-900'>Real-time payment tracking</li>
              <li className='text-gray-900'>Automated reconciliation</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}
