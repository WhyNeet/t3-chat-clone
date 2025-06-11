import { forwardRef, type ComponentProps } from "react";
import { Role, type ChatMessage } from "../lib/model/message";
import { cn } from "./utils";
import Markdown from "react-markdown";
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter'

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
        <Markdown components={{
          code(props) {
            const { children, className, node, ...rest } = props
            const match = /language-(\w+)/.exec(className || '')
            return match ? (
              <SyntaxHighlighter
                {...rest}
                PreTag="div"
                children={String(children).replace(/\n$/, '')}
                language={match[1]}
              />
            ) : (
              <code {...rest} className={className}>
                {children}
              </code>
            )
          }
        }}>{message.content}</Markdown>
      </div>
    </div>
  );
});
