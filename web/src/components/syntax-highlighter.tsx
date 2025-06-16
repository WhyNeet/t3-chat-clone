import type { ExtraProps } from "react-markdown";
import { Prism } from "react-syntax-highlighter";


export default function SyntaxHighlighter({ className, children, node, language, ...rest }: React.ClassAttributes<HTMLElement> & React.HTMLAttributes<HTMLElement> & ExtraProps & { language: string }) {
  /* @ts-ignore */
  return <Prism
    {...rest}
    PreTag="figure"
    children={String(children).replace(/\n$/, "")}
    language={language}
  />;
}