import { forwardRef, type ComponentProps } from "react";
import { cn } from "../utils";

export const Input = forwardRef<HTMLInputElement, ComponentProps<"input">>(
  ({ className, ...props }, ref) => {
    return (
      <input
        ref={ref}
        className={cn("rounded-lg border border-black/10 placeholder:text-gray-400 placeholder:font-display p-2 bg-black/5 outline-none focus:border-pink-500 transition font-sans", className)}
        {...props}
      />
    );
  },
);
