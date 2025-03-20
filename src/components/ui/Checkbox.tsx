import React, { InputHTMLAttributes, forwardRef } from 'react';
import cn from 'classnames';

export interface CheckboxProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'type'> {
  label?: React.ReactNode;
  error?: string;
  className?: string;
}

export const Checkbox = forwardRef<HTMLInputElement, CheckboxProps>(
  ({ label, error, className, id, ...props }, ref) => {
    return (
      <div className={cn('flex flex-col', className)}>
        <div className="flex items-start">
          <div className="flex items-center h-5">
            <input
              ref={ref}
              type="checkbox"
              id={id}
              className={cn(
                'h-4 w-4 rounded border-gray-300',
                'text-primary focus:ring-primary',
                error && 'border-red-500',
                props.disabled && 'cursor-not-allowed opacity-50'
              )}
              {...props}
            />
          </div>
          {label && (
            <label
              htmlFor={id}
              className={cn(
                'ml-2 block text-sm text-gray-900',
                props.disabled && 'cursor-not-allowed opacity-50'
              )}
            >
              {label}
            </label>
          )}
        </div>
        {error && <p className="mt-1 text-sm text-red-500">{error}</p>}
      </div>
    );
  }
);

Checkbox.displayName = 'Checkbox'; 