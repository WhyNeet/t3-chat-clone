import { useState } from "react";
import { Button } from "../components/ui/button";
import { ChatsSidebar } from "../components/chats-sidebar";
import { ChevronRight } from "lucide-react";
import { Outlet } from "react-router";
import { Prompt } from "../components/prompt";

export function App() {
  const [sidebarOpen, setSidebarOpen] = useState(true);

  return (
    <div
      className={`h-full w-full flex items-stretch transition-all relative duration-200 bg-pink-200 ${sidebarOpen ? "pt-1" : ""}`}
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
        <div className="absolute bottom-0 flex justify-center inset-x-0 px-1 md:px-5 lg:px-10">
          <Prompt />
        </div>
      </div>
    </div>
  );
}

export default App;
