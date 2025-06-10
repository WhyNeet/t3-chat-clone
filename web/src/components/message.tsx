import { forwardRef, type ComponentProps } from "react";
import { Role, type ChatMessage } from "../lib/model/message";
import { cn } from "./utils";
import Markdown from "react-markdown";

export const Message = forwardRef<
  HTMLDivElement,
  ComponentProps<"div"> & { message: ChatMessage }
>(({ message, className, ...props }, ref) => {
  return (
    <div
      className={cn(
        `mb-2 p-4 border border-transparent hover:border-pink-950 rounded-lg ${message.role === Role.User ? "self-end" : ""}`,
        className,
      )}
      {...props}
      ref={ref}
    >
      <div className="prose">
        <Markdown>{message.content}</Markdown>
      </div>
    </div>
  );
});
