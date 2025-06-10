import { forwardRef, type ComponentProps } from "react";

export const Icon = forwardRef<HTMLSpanElement, ComponentProps<"span">>(({ className, ...props }, ref) => {
  return <span className={`${className} material-symbols-outlined`} {...props} ref={ref}></span>
})
