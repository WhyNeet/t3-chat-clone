import { useEffect, useRef } from "react";
import { Button } from "../components/ui/button";
import { ChatsSidebar, useSidebarStore } from "../components/chats-sidebar";
import { ChevronRight } from "lucide-react";
import { Outlet, useLocation, useNavigate } from "react-router";
import { cn } from "../components/utils";
import { Logo } from "../components/logo";

export function App() {
  const navigate = useNavigate();
  const location = useLocation();
  const { isOpen: sidebarOpen, toggle, setIsOpen: setSidebarOpen } = useSidebarStore();
  const viewRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    localStorage.setItem("sidebar-open", Number(sidebarOpen).toString())

    if (!viewRef.current) return;
    viewRef.current.classList.add("transition-all!");
    const id = setTimeout(() => {
      viewRef.current!.classList.remove("transition-all!");
    }, 201);
    return () => clearTimeout(id);
  }, [sidebarOpen]);

  return (
    <div
      className={`h-full w-full overflow-hidden flex relative transition-[padding-top] duration-200 bg-pink-200 ${sidebarOpen ? "md:pt-1" : ""}`}
    >
      <div
        className={`transition-all duration-200 ease-in-out fixed md:relative left-0 top-1 bottom-0 z-50 bg-pink-200 rounded-tr-2xl ${sidebarOpen ? "min-w-72 w-72" : "min-w-0 w-0"}`}
      >
        <ChatsSidebar />
      </div>
      <div onClick={() => setSidebarOpen(false)} className={cn("md:hidden bg-pink-950/30 absolute inset-0 z-20 transition-all", sidebarOpen ? "opacity-100" : "opacity-0 pointer-events-none")}></div>
      <div
        ref={viewRef}
        className={`h-full bg-white transition-none duration-200 overflow-hidden ${sidebarOpen ? "md:rounded-tl-3xl w-screen md:w-[calc(100vw-288px)]" : "w-full"} relative`}
      >
        <div className="fixed top-3 left-3 z-50 flex gap-2">
          <Button
            onClick={toggle}
            intent="ghost"
            size="square"
            rounded="circle"
            className={cn("transition duration-200", sidebarOpen ? "hover:bg-pink-50" : "bg-pink-300/20 hover:bg-pink-400/20 backdrop-blur-2xl")}
          >
            <ChevronRight
              className={`h-5 w-5 text-pink-900 transition duration-200 ${sidebarOpen ? "rotate-180" : "rotate-0"}`}
            />
          </Button>
          <Button
            onClick={() => navigate("/")}
            intent="ghost"
            size="square"
            rounded="circle"
            className={cn("bg-pink-300/20 hover:bg-pink-400/20 backdrop-blur-2xl transition duration-200", sidebarOpen || location.pathname === "/" ? "opacity-0 scale-50 pointer-events-none" : "opacity-100 scale-100")}
          >
            <Logo
              className="h-5 w-5 text-pink-900 transition duration-200"
            />
          </Button>
        </div>
        <Outlet />
      </div>
    </div>
  );
}

export default App;
