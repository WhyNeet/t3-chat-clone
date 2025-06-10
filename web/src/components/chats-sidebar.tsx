import { NavLink } from "react-router";
import { Logo } from "./logo";
import { useChatsStore } from "../lib/state/chats";
import { AuthState, useAuthStore } from "../lib/state/auth";
import { Button } from "./ui/button";
import { Loader } from "./ui/loader";
import { Ellipsis } from "lucide-react";

export function ChatsSidebar() {
  const isLoggedIn = useAuthStore((state) => !!state.user);
  const isUserLoading = useAuthStore(
    (state) => state.state === AuthState.Loading,
  );
  const chats = useChatsStore((state) => state.chats);
  const isLoading = useChatsStore(state => state.isFetching);

  return (
    <aside className="min-w-72 rounded-tr-3xl h-[calc(100vh-0.25rem)] absolute right-0 flex flex-col">
      <div className="p-6 pt-3 flex gap-4 items-center text-lg font-bold font-display justify-center">
        <Logo className="h-6 w-6 text-pink-500" />
        Why Chat
      </div>
      <div className="pb-6 px-3 h-full overflow-scroll flex flex-col">
        {isLoading ? (
          <div className="h-full w-full flex items-center justify-center">
            <Loader className="text-pink-900" />
          </div>
        ) : chats ? (
          <>
            <NavLink to="/" className="mb-2"><Button className="w-full" intent="primary">New Chat</Button></NavLink>
            {Object.values(chats).map(({ chat }) => (
              <NavLink
                key={chat.id}
                to={`/chat/${chat.id}`}
                className={({ isActive }) =>
                  `w-full rounded-lg border-2 border-transparent px-4 py-2 font-medium font-display group flex items-center justify-between ${isActive ? "border-pink-800 bg-pink-50" : ""}`
                }
              >
                {chat.name ?? "New Chat"}
                <div className="hidden group-hover:block">
                  <Ellipsis className="h-6 w-6 stroke-2" />
                </div>
              </NavLink>
            ))}
          </>
        ) : (
          <div>Error.</div>
        )}
      </div>
      {isUserLoading ? null : isLoggedIn ? null : (
        <div className="rounded-lg border border-pink-800 bg-pink-50 px-4 py-3 m-2">
          <p className="font-display text-sm mb-3 text-pink-900">Log in to access all features of Why&nbsp;Chat.</p>
          <NavLink to="/auth/login">
            <Button intent="primary" className="w-full">Log In</Button>
          </NavLink>
        </div>
      )}
    </aside>
  );
}
