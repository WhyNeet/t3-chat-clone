import { cva, type VariantProps } from "class-variance-authority";
import { forwardRef, type ComponentProps } from "react";

export const button = cva(["outline-none", "cursor-pointer", "transition", "flex", "items-center justify-center", "gap-2"],
  {
    variants: {
      intent: {
        primary: ["bg-pink-600", "hover:bg-pink-700", "font-display", "text-white", "focus:bg-pink-700", "active:bg-pink-700"],
        ghost: ["bg-transparent", "hover:bg-black/10", "text-black", "font-display"]
      },
      size: {
        medium: ["px-2", "py-2"],
        square: ["h-9", "w-9"],
      },
      rounded: {
        default: ["rounded-lg"],
        circle: ["rounded-full"]
      }
    },
    defaultVariants: {
      size: "medium",
      rounded: "default"
    }
  })

export type ButtonAttributes = ComponentProps<"button"> & VariantProps<typeof button>;

export const Button = forwardRef<HTMLButtonElement, ButtonAttributes>(
  ({ className, intent, size, rounded, ...props }, ref) => {
    return (
      <button
        ref={ref}
        className={`${button({ intent, size, rounded })} ${className}`}
        {...props}
      />
    );
  },
);
