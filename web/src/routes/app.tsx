import { useState } from "react";
import { Button } from "../components/ui/button";
import { ChatsSidebar } from "../components/chats-sidebar";
import { ChevronRight, Command, SendHorizonal } from "lucide-react";
import { Outlet } from "react-router";

export function App() {
  const [sidebarOpen, setSidebarOpen] = useState(true);
  const [message, setMessage] = useState("");

  const runPrompt = async () => {
    // const res = await fetch("http://localhost:8080/completions/prompt", {
    //   method: "POST",
    //   headers: {
    //     Accept: "application/json",
    //     "Content-Type": "application/json",
    //   },
    //   body: JSON.stringify({ message, model: "google/gemma-3-27b-it:free" }),
    // });
    // const { stream_id } = await res.json();

    // const events = new EventSource(
    //   `http://localhost:8080/completions/prompt/sse/${stream_id}`,
    // );

    // events.addEventListener("message", (message: MessageEvent<string>) => {
    //   const data: CompletionData = JSON.parse(message.data);
    //   if (isControl(data)) {
    //     switch (data.control) {
    //       case "done":
    //         events.close();
    //         return;
    //     }
    //   }
    //   setResult((prev) => prev + data.content);
    // });
  };

  return (
    <div
      className={`h-full w-full flex items-stretch transition-all duration-200 bg-pink-200 ${sidebarOpen ? "pt-1" : ""}`}
    >
      <div
        className={`transition-all duration-200 ease-in-out relative ${sidebarOpen ? "min-w-72 w-72" : "min-w-0 w-0"}`}
      >
        <ChatsSidebar />
      </div>
      <div
        className={`h-full w-full bg-white transition-all duration-200 ${sidebarOpen ? "rounded-tl-3xl" : ""} relative`}
      >
        <div className="fixed top-3 left-3">
          <Button
            onClick={() => setSidebarOpen((prev) => !prev)}
            intent="ghost"
            size="square"
            rounded="circle"
            className="hover:bg-pink-50"
          >
            <ChevronRight
              className={`h-5 w-5 transition duration-200 ${sidebarOpen ? "rotate-180" : "rotate-0"}`}
            />
          </Button>
        </div>
        <Outlet />
        <div className="absolute bottom-0 inset-x-0 md:inset-x-40 bg-pink-50/50 backdrop-blur-3xl border border-b-0 border-pink-200 p-2 rounded-t-lg flex gap-2">
          <textarea value={message} onInput={(e) => setMessage(e.currentTarget.value)} className="resize-none w-full outline-none border-0 placeholder:font-display placeholder:text-pink-900/40 pl-2 pt-2" placeholder="Message Gemini 2.0 Flash" />
          <div className="text-center">
            <Button intent="primary" className="h-12 w-14">
              <SendHorizonal className="text-pink-950 h-5 w-5" />
            </Button>
            <p className="text-xs font-display font-semibold text-black/40 flex items-center">
              <Command className="h-3 w-3 stroke-3 mr-0.5" />
              {" + "}Enter
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
