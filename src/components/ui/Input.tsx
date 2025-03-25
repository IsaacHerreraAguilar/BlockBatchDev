import React, { InputHTMLAttributes, forwardRef } from 'react';
import cn from 'classnames';

export interface InputProps extends InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  labelComponent?: React.ReactNode;
  error?: string;
  fullWidth?: boolean;
  className?: string;
}

export const Input = forwardRef<HTMLInputElement, InputProps>(
  (
    { label, labelComponent, error, fullWidth = true, className, ...props },
    ref
  ) => {
    return (
      <div
        className={cn('flex flex-col gap-1', fullWidth && 'w-full', className)}
      >
        {labelComponent
          ? labelComponent
          : label && (
              <label
                className='text-sm font-medium text-gray-700'
                htmlFor={props.id}
              >
                {label}
              </label>
            )}
        <input
          ref={ref}
          className={cn(
            'px-4 py-2 rounded-lg border border-gray-300',
            'focus:outline-none focus:ring-2 focus:primary focus:border-transparent',
            'transition-all duration-200',
            'placeholder:text-gray-400',
            error ? 'border-red-500' : 'border-gray-300',
            fullWidth && 'w-full'
          )}
          {...props}
        />
        {error && <span className='text-sm text-red-500'>{error}</span>}
      </div>
    );
  }
);

Input.displayName = 'Input';
