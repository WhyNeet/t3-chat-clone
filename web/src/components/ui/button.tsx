import { cva, type VariantProps } from "class-variance-authority";
import { forwardRef, type ComponentProps } from "react";
import { cn } from "../utils";

export const button = cva(
  [
    "outline-none",
    "cursor-pointer",
    "transition-all",
    "duration-150",
    "flex",
    "items-center",
    "justify-center",
    "gap-2"
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
          "disabled:bg-slate-300",
          "disabled:border-slate-400",
          "disabled:text-slate-400",
          "disabled:hover:bg-slate-300",
          "disabled:cursor-not-allowed"
        ],
        secondary: [
          "bg-pink-100",
          "hover:bg-pink-200",
          "font-display",
          "text-pink-950",
          "font-semibold",
          "border",
          "border-pink-900/20",
          "disabled:bg-slate-300",
          "disabled:border-slate-400",
          "disabled:text-slate-400",
          "disabled:hover:bg-slate-300",
          "disabled:cursor-not-allowed"
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
        small: ["text-sm", "px-2", "py-1"]
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
        className={cn(button({ intent, size, rounded }), className)}
        {...props}
      />
    );
  },
);
