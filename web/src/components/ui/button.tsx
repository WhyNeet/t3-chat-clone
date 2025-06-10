import { cva, type VariantProps } from "class-variance-authority";
import { forwardRef, type ComponentProps } from "react";

export const button = cva(
  [
    "outline-none",
    "cursor-pointer",
    "transition-all",
    "duration-150",
    "flex",
    "items-center",
    "justify-center",
    "gap-2",
    "disabled:opacity-80"
  ],
  {
    variants: {
      intent: {
        primary: [
          "bg-pink-500",
          "hover:bg-pink-600",
          "font-display",
          "text-pink-950",
          "font-semibold",
          "border-2",
          "border-pink-900",
          "disabled:hover;bg-pink-500"
        ],
        ghost: [
          "bg-transparent",
          "hover:bg-black/10",
          "text-black",
          "font-display",
        ],
      },
      size: {
        medium: ["px-2", "py-2"],
        square: ["h-9", "w-9"],
      },
      rounded: {
        default: ["rounded-lg"],
        circle: ["rounded-full"],
      },
    },
    defaultVariants: {
      size: "medium",
      rounded: "default",
    },
  },
);

export type ButtonAttributes = ComponentProps<"button"> &
  VariantProps<typeof button>;

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
