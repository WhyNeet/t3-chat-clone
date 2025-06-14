import { forwardRef, useState, type ComponentProps } from "react";
import { Role, type ChatMessage } from "../lib/model/message";
import { cn } from "./utils";
import Markdown from "react-markdown";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { ChevronDownIcon } from "lucide-react";

export const Message = forwardRef<
  HTMLDivElement,
  ComponentProps<"div"> & { message: ChatMessage }
>(({ message, className, ...props }, ref) => {
  const [showThoughts, setShowThoughts] = useState(true);

  return (
    <div
      className={cn(
        `mb-2 p-4 border border-transparent relative hover:border-pink-950 rounded-lg ${message.role === Role.User ? "self-end" : ""}`,
        className,
      )}
      {...props}
      ref={ref}
    >
      {message.model ? <div className="absolute px-2 py-1 rounded-lg bg-white h-8 -bottom-4 left-4 text-sm border border-pink-900/10 font-display text-pink-900/80">{message.model}</div> : null}
      {message.reasoning ? (
        <div
          className={cn(
            "w-full rounded-lg overflow-hidden",
            message.content.length ? "mb-6" : "",
          )}
        >
          <div
            className={cn(
              "px-4 py-2 border-b font-display font-medium flex items-center gap-1 cursor-pointer border-b-pink-900/20 transition bg-[#f5f2f0] select-none",
              showThoughts ? "" : "border-b-transparent",
            )}
            onClick={() => setShowThoughts((prev) => !prev)}
          >
            Model Thoughts
            <ChevronDownIcon
              className={cn(
                "h-4 w-4 transition",
                showThoughts ? "rotate-180" : "",
              )}
            />
          </div>
          <div
            className={cn(
              "px-4 py-2 transition-all overflow-hidden bg-pink-950/10 whitespace-pre-line",
              showThoughts ? "h-fit" : "h-0 py-0",
            )}
          >
            {message.reasoning}
          </div>
        </div>
      ) : null}
      <div className="prose wrap-anywhere ">
        <Markdown
          components={{
            code(props) {
              /* @ts-ignore */
              const { children, className, node, ...rest } = props;
              const match = /language-(\w+)/.exec(className || "");
              return match ? (
                /* @ts-ignore */
                <SyntaxHighlighter
                  {...rest}
                  PreTag="figure"
                  children={String(children).replace(/\n$/, "")}
                  language={match[1]}
                />
              ) : (
                <code {...rest} className={className}>
                  {children}
                </code>
              );
            },
          }}
        >
          {message.content}
        </Markdown>
      </div>
      {message.model ? <div className="h-2"></div> : null}
    </div>
  );
});
