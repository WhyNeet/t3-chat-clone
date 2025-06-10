import { forwardRef, type ComponentProps } from "react";

export const Skeleton = forwardRef<HTMLDivElement, ComponentProps<"div">>(({ className, ...props }, ref) => {
  return <div className={`animate-pulse rounded-lg bg-black/10 ${className}`} {...props} ref={ref} />
});
