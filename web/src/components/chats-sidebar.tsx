import { NavLink } from "react-router";
import { Logo } from "./logo";
import { useChatsStore } from "../lib/state/chats";
import { AuthState, useAuthStore } from "../lib/state/auth";
import { Button } from "./ui/button";
import { Loader } from "./ui/loader";
import { Cog, Ellipsis, LogOut, Pen, Trash } from "lucide-react";
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "./ui/dropdown-menu";
import { deleteChat } from "../lib/api/chats";
import { useNavigate } from "react-router";
import { logout } from "../lib/api/auth";
import { useState } from "react";

export function ChatsSidebar() {
  const navigate = useNavigate();
  const isLoggedIn = useAuthStore((state) => !!state.user);
  const isUserLoading = useAuthStore(
    (state) => state.state === AuthState.Loading,
  );
  const deleteChatState = useChatsStore(state => state.deleteChat);
  const chats = useChatsStore((state) => state.chats);
  const isLoading = useChatsStore((state) => state.isFetching);
  const user = useAuthStore((state) => state.user);
  const [isLoggingOut, setIsLoggingOut] = useState(false);

  const handleDeleteChat = async (id: string) => {
    await deleteChat(id);
    deleteChatState(id);
    navigate("/");
  }

  const handleLogOut = async () => {
    setIsLoggingOut(true);
    await logout();
    navigate("/");
    window.location.reload();
  }

  return (
    <aside className="min-w-72 max-w-72 rounded-tr-3xl h-[calc(100vh-0.25rem)] absolute right-0 flex flex-col">
      <div className="p-6 pt-3 flex gap-4 items-center text-lg font-bold font-display justify-center">
        <Logo className="h-6 w-6 text-pink-500" />
        Why Chat
      </div>
      {user && !isLoading ? <NavLink to="/" className="mb-2 mx-3">
        <Button className="w-full" intent="primary">
          New Chat
        </Button>
      </NavLink> : null}
      <div className="pb-20 px-3 h-full overflow-y-scroll flex flex-col">
        {user ? (
          isLoading ? (
            <div className="h-full w-full flex items-center justify-center">
              <Loader className="text-pink-900" />
            </div>
          ) : chats ? (
            <>
              <div className="flex flex-col-reverse">
                {Object.values(chats)
                  .reverse()
                  .map(({ chat, streaming }) => (
                    <NavLink
                      key={chat?.id}
                      to={`/chat/${chat?.id}`}
                      className={({ isActive }) =>
                        `w-full rounded-lg border-2 border-transparent px-4 py-2 font-medium font-display group flex items-center justify-between ${isActive ? "border-pink-800 bg-pink-50" : ""}`
                      }
                    >
                      <div className="whitespace-nowrap max-w-10/12 overflow-hidden overflow-ellipsis">{chat?.name ?? "New Chat"}</div>
                      <div className="flex items-center gap-2">
                        {streaming ? (
                          <Loader className="h-5 w-5 text-pink-900" />
                        ) : (
                          <DropdownMenu>
                            <DropdownMenuTrigger asChild>
                              <button className="cursor-pointer">
                                <Ellipsis className="h-6 w-6 stroke-2" />
                              </button>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent className="w-56 font-display text-pink-950" align="start">
                              <DropdownMenuItem onClick={() => { }}>
                                <Pen className="h-4 w-4" />
                                Rename
                              </DropdownMenuItem>
                              <DropdownMenuItem onClick={(e) => {
                                e.stopPropagation();
                                handleDeleteChat(chat.id);
                              }} className="text-red-500 hover:text-white hover:bg-red-500!">
                                <Trash className="h-4 w-4" />
                                Delete
                              </DropdownMenuItem>
                            </DropdownMenuContent>
                          </DropdownMenu>
                        )}
                      </div>
                    </NavLink>
                  ))}
              </div>
            </>
          ) : (
            <div>Error.</div>
          )
        ) : (
          <NavLink to="/" className="mb-2">
            <Button className="w-full" intent="primary">
              New Chat
            </Button>
          </NavLink>
        )}
      </div>
      {isLoggedIn ? <div className="flex items-center justify-between rounded-full border border-pink-900/20 p-2 absolute bottom-2 inset-x-3 bg-pink-200/20 backdrop-blur-xl">
        <NavLink to="/settings">
          <Button intent="secondary" rounded="circle">
            <Cog className="h-5 w-5" />
          </Button>
        </NavLink>
        <Button onClick={() => handleLogOut()} intent="secondary" rounded="circle">
          {isLoggingOut ? <Loader className="h-5 w-5" /> : <LogOut className="h-5 w-5" />}
        </Button>
      </div> : null}
      {isUserLoading ? null : isLoggedIn ? null : (
        <div className="rounded-lg border border-pink-800 bg-pink-50 px-4 py-3 m-2">
          <p className="font-display text-sm mb-3 text-pink-900">
            Log in to access all features of Why&nbsp;Chat.{" "}
            {isNotChrome() ? "Only tested in Chrome." : null}
          </p>
          <NavLink to="/auth/login">
            <Button intent="primary" className="w-full">
              Log In
            </Button>
          </NavLink>
        </div>
      )}
    </aside>
  );
}

const isNotChrome = () => {
  return (
    !navigator.userAgent.includes("Chrome/") &&
    !navigator.userAgent.includes("Chromium/")
  );
};
