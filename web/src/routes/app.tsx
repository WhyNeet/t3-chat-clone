import { useEffect, useState } from "react";
import { isControl, type CompletionData } from "../lib/completion";
import Markdown from "react-markdown";
import { useNavigate } from "react-router";
import { AuthState, useAuthStore } from "../lib/state/auth";
import { Logo } from "../components/logo";
import { Button } from "../components/ui/button";
import { Bars3Icon } from "@heroicons/react/24/solid";
import { ChevronRightIcon } from "@heroicons/react/24/solid";

export function App() {
  const navigate = useNavigate();
  const user = useAuthStore((state) => state.user);
  const authLoading = useAuthStore(state => state.state === AuthState.Loading);

  useEffect(() => {
    if (!authLoading && !user) navigate("/auth/login");
  }, [user, navigate, authLoading]);

  const [sidebarOpen, setSidebarOpen] = useState(true);
  const [currentChat, setCurrentChat] = useState<string | null>(null);
  const [message, setMessage] = useState("");
  const [result, setResult] = useState("");

  const runPrompt = async () => {
    const res = await fetch("http://localhost:8080/completions/prompt", {
      method: "POST",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ message, model: "google/gemma-3-27b-it:free" }),
    });
    const { stream_id } = await res.json();

    const events = new EventSource(
      `http://localhost:8080/completions/prompt/sse/${stream_id}`,
    );

    events.addEventListener("message", (message: MessageEvent<string>) => {
      const data: CompletionData = JSON.parse(message.data);
      if (isControl(data)) {
        switch (data.control) {
          case "done":
            events.close();
            return;
        }
      }
      setResult((prev) => prev + data.content);
    });
  };

  return (
    <div className="h-full w-full flex items-stretch bg-pink-200 p-1 pl-0">
      <div className={`transition-all duration-150 relative ${sidebarOpen ? "min-w-72 w-72" : "min-w-0 w-0"}`}>
        <aside className="min-w-72 bg-white rounded-r-3xl h-full absolute right-0">
          <div className="p-6 flex gap-6 items-center text-xl font-bold font-display">
            <Logo className="h-8 w-8 text-pink-500" />
            Why Chat
          </div>
          <div className="p-6">chats</div>
        </aside>
      </div>
      <div className="h-full w-full bg-white rounded-3xl relative ml-1">
        <div className="absolute top-3 left-3">
          <Button onClick={() => setSidebarOpen(prev => !prev)} intent="ghost" size="square" rounded="circle">
            <ChevronRightIcon className={`h-5 w-5 ${sidebarOpen ? "rotate-180" : "rotate-0"}`} />
          </Button>
        </div>
        {currentChat ? <div>chat</div> : <div className="h-full w-full flex items-center justify-center flex-col">
          <Logo className="h-12 w-12 mb-6 text-black/20" />
          <p className="font-display text-sm text-black/20 font-medium">No chat selected.</p>
        </div>}
      </div>
    </div>
  );
}

export default App;
