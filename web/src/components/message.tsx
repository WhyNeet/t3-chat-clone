import { forwardRef, useState, type ComponentProps } from "react";
import { Role, type ChatMessage } from "../lib/model/message";
import { cn } from "./utils";
import Markdown from "react-markdown";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { ChevronDownIcon, Database, FileText } from "lucide-react";
import { getFileUri } from "../lib/api/files";
import { Tooltip, TooltipTrigger, TooltipContent } from "./ui/tooltip";
import { Popover, PopoverContent } from "./ui/popover";
import { PopoverTrigger } from "@radix-ui/react-popover";

export const Message = forwardRef<
  HTMLDivElement,
  ComponentProps<"div"> & { message: ChatMessage }
>(({ message, className, ...props }, ref) => {
  const [showThoughts, setShowThoughts] = useState(true);

  return (
    <div
      className={cn(
        `mb-2 p-4 border border-transparent relative hover:border-pink-950/20 rounded-lg ${message.role === Role.User ? "self-end" : ""}`,
        message.role === Role.User ? "bg-pink-50" : "",
        className,
        message.model ? "mb-6" : ""
      )}
      {...props}
      ref={ref}
    >
      {message.updated_memory ? <Popover>
        <PopoverTrigger asChild>
          <div className="mb-4 font-display text-pink-300 brightness-75 flex items-center gap-2 text-sm font-medium w-fit">
            <Database className="h-6 w-6" />
            Memory Updated
          </div>
        </PopoverTrigger>
        <PopoverContent className="bg-pink-50 border-pink-900/20 text-sm font-medium font-display px-2 py-2 w-fit text-pink-900">
          {message.updated_memory}
        </PopoverContent>
      </Popover> : null}
      {message.content.length > 1 ? <div className="flex gap-2 mb-2">
        {message.content.slice(1).map(content => {
          switch (content.type) {
            case "Image":
              return <div key={content.id} className="rounded-lg overflow-hidden max-h-64 min-w-64 w-full flex items-center bg-white justify-center border-2 border-pink-900/20">
                <img src={getFileUri(message.chat_id, content.id)} className="h-auto max-h-64 w-full bg-contain bg-no-repeat bg-center" />
              </div>
            case "Pdf":
              return <a href={getFileUri(message.chat_id, content.id)} target="_blank" key={content.id} className="rounded-lg overflow-hidden p-4 border border-pink-900/20 w-64 bg-white flex justify-start items-center gap-2 text-sm text-pink-900! font-medium font-display">
                <FileText className="h-6 w-6 text-pink-900" />
                PDF Attachment
              </a>
          }
        })}</div> : null}
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
          {(message.content[0] as unknown as Record<string, string>)["value"]}
        </Markdown>
      </div>
      {message.model ? <div className="h-2"></div> : null}
    </div>
  );
});
